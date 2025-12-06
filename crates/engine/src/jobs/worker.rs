use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

use crate::fs::conflict::ConflictPolicy;
use crate::fs::ops;

use super::job::{JobSpec, JobKind, JobId};
use super::scheduler::WorkerResult;
use super::cancel::CancelRegistry;

pub enum WorkerMsg {
    Result(WorkerResult),
    Progress(JobId, u64, u64),
}

/// Spawns a single worker task and loops forever.
pub fn start_worker(
    rx: Arc<tokio::sync::Mutex<Receiver<JobSpec>>>,
    tx: Sender<WorkerMsg>,
    cancel: Arc<CancelRegistry>,
) {
    tokio::spawn(async move {
        loop {
            let job = {
                let mut guard = rx.lock().await;
                guard.recv().await
            };

            let job = match job {
                Some(j) => j,
                None => break,
            };

            let id = job.id;
            cancel.register(id);

            let result = execute_job(&cancel, &tx, job).await;

            cancel.unregister(id);

            let _ = tx.send(WorkerMsg::Result(result)).await;
        }
    });
}

/// Dispatch execution based on job kind
async fn execute_job(
    cancel: &CancelRegistry,
    tx: &Sender<WorkerMsg>,
    job: JobSpec
) -> WorkerResult {
    let id = job.id;

    match job.kind {
        JobKind::Copy { from, to } => {
            let src = std::path::Path::new(&from);
            let result = if src.is_dir() {
                ops::copy_tree(
                    id,
                    cancel,
                    src,
                    to.as_ref(),
                    ConflictPolicy::Rename,
                    |_| {}
                ).await.map(|_| to.clone())
            } else {
                ops::copy_file(
                    id,
                    cancel,
                    src,
                    to.as_ref(),
                    ConflictPolicy::Rename,
                    |done, total| {
                        let _ = tx.send(WorkerMsg::Progress(id, done, total));
                    }
                ).await.map(|_| to.clone())
            };

            match result {
                Ok(_) => WorkerResult::Success(id),
                Err(e) => fs_error(id, e),
            }
        }

        JobKind::IndexScan { root } => {
            scan_with_progress(id, &root, cancel, tx).await
        }

        JobKind::Move { from, to } => {
            match ops::move_path(
                from.as_ref(),
                to.as_ref(),
                ConflictPolicy::Rename
            ).await {
                Ok(_) => WorkerResult::Success(id),
                Err(e) => fs_error(id, e),
            }
        }

        JobKind::DeleteTree { target } => {
            ops::delete_tree(id, cancel, target.as_ref())
                .await
                .map(|_| WorkerResult::Success(id))
                .unwrap_or_else(|e| fs_error(id, e))
        }

        JobKind::RebuildIndex => WorkerResult::Success(id),
        JobKind::Thumbnail { .. } => WorkerResult::Success(id),
    }
}

fn fs_error(id: JobId, err: crate::fs::ops::FsError) -> WorkerResult {
    match err {
        crate::fs::ops::FsError::Cancelled(_) => WorkerResult::Cancelled(id),
        crate::fs::ops::FsError::Io(e) => WorkerResult::Failed(id, e.to_string()),
    }
}

/* =============================
   REAL IMPLEMENTATIONS
   ============================= */

async fn copy_with_progress(
    id: JobId,
    from: &str,
    to: &str,
    cancel: &CancelRegistry,
    tx: &Sender<WorkerMsg>,
) -> WorkerResult {

    let mut src = match tokio::fs::File::open(from).await {
        Ok(f) => f,
        Err(e) => return WorkerResult::Failed(id, e.to_string()),
    };

    let meta = match src.metadata().await {
        Ok(m) => m,
        Err(e) => return WorkerResult::Failed(id, e.to_string()),
    };

    let total = meta.len();
    let mut dst = match tokio::fs::File::create(to).await {
        Ok(f) => f,
        Err(e) => return WorkerResult::Failed(id, e.to_string()),
    };

    let mut buf = vec![0u8; 64 * 1024];
    let mut done: u64 = 0;

    loop {
        if cancel.is_cancelled(id) {
            return WorkerResult::Cancelled(id);
        }

        let n = match src.read(&mut buf).await {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => return WorkerResult::Failed(id, e.to_string()),
        };

        if let Err(e) = dst.write_all(&buf[..n]).await {
            return WorkerResult::Failed(id, e.to_string());
        }

        done += n as u64;

        let _ = tx.send(WorkerMsg::Progress(id, done, total)).await;
    }

    WorkerResult::Success(id)
}

async fn scan_with_progress(
    id: JobId,
    root: &str,
    cancel: &CancelRegistry,
    tx: &Sender<WorkerMsg>,
) -> WorkerResult {
    use tokio::fs;
    use std::path::PathBuf;

    let mut stack = vec![PathBuf::from(root)];
    let mut done = 0u64;
    let mut total = 0u64;

    // COUNT PHASE
    {
        let mut s = vec![PathBuf::from(root)];
        while let Some(p) = s.pop() {
            if cancel.is_cancelled(id) {
                return WorkerResult::Cancelled(id);
            }

            if let Ok(meta) = fs::metadata(&p).await {
                if meta.is_dir() {
                    if let Ok(mut rd) = fs::read_dir(&p).await {
                        while let Ok(Some(e)) = rd.next_entry().await {
                            s.push(e.path());
                        }
                    }
                } else {
                    total += 1;
                }
            }
        }
    }

    // SCAN PHASE
    while let Some(p) = stack.pop() {
        if cancel.is_cancelled(id) {
            return WorkerResult::Cancelled(id);
        }

        if let Ok(meta) = fs::metadata(&p).await {
            if meta.is_dir() {
                if let Ok(mut rd) = fs::read_dir(&p).await {
                    while let Ok(Some(e)) = rd.next_entry().await {
                        stack.push(e.path());
                    }
                }
            } else {
                done += 1;
                let _ = tx.send(WorkerMsg::Progress(id, done, total)).await;
            }
        }
    }

    WorkerResult::Success(id)
}
