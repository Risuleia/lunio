use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::fs::conflict::{ConflictPolicy, ConflictResolution, resolve_path};
use crate::jobs::{job::JobId, cancel::CancelRegistry};

#[derive(Debug)]
pub enum FsError {
    Io(std::io::Error),
    Cancelled(JobId),
}
impl From<std::io::Error> for FsError {
    fn from(e: std::io::Error) -> Self {
        FsError::Io(e)
    }
}

/* =============================
   COPY FILE (WITH CANCEL)
============================= */

pub async fn copy_file(
    id: JobId,
    cancel: &CancelRegistry,
    from: &Path,
    to: &Path,
    policy: ConflictPolicy,
    mut progress: impl FnMut(u64, u64) + Send,
) -> Result<PathBuf, FsError> {

    let mut src = fs::File::open(from).await?;
    let total = src.metadata().await?.len();

    let resolved = match resolve_path(to.to_path_buf(), policy).await {
        ConflictResolution::Use(p) => p,
        ConflictResolution::Skip => return Ok(to.to_path_buf()),
        ConflictResolution::Error(e) => return Err(FsError::Io(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            e,
        ))),
    };

    let mut dst = fs::File::create(&resolved).await?;
    let mut buf = vec![0u8; 128 * 1024];
    let mut done = 0;

    loop {
        if cancel.is_cancelled(id) {
            let _ = fs::remove_file(to).await;
            return Err(FsError::Cancelled(id));
        }

        let n = src.read(&mut buf).await?;
        if n == 0 {
            break;
        }

        dst.write_all(&buf[..n]).await?;
        done += n as u64;

        progress(done, total);
    }

    Ok(resolved)
}

pub async fn copy_tree(
    id: JobId,
    cancel: &CancelRegistry,
    from: &Path,
    to: &Path,
    policy: ConflictPolicy,
    mut progress: impl FnMut(PathBuf) + Send,
) -> Result<PathBuf, FsError> {

    let dest_root = match resolve_path(to.to_path_buf(), policy).await {
        ConflictResolution::Use(p) => p,
        ConflictResolution::Skip => return Ok(from.to_path_buf()),
        ConflictResolution::Error(e) => return Err(FsError::Io(
            std::io::Error::new(std::io::ErrorKind::AlreadyExists, e)
        )),
    };

    fs::create_dir_all(&dest_root).await?;

    let mut stack = vec![(from.to_path_buf(), dest_root.clone())];

    while let Some((src_dir, dst_dir)) = stack.pop() {

        if cancel.is_cancelled(id) {
            return Err(FsError::Cancelled(id));
        }

        let mut rd = fs::read_dir(&src_dir).await?;

        while let Some(entry) = rd.next_entry().await? {

            let src_path = entry.path();
            let tgt_path = dst_dir.join(entry.file_name());

            let meta = entry.metadata().await?;

            if meta.is_dir() {
                fs::create_dir_all(&tgt_path).await?;
                stack.push((src_path, tgt_path.clone()));
            }
            else {
                copy_file(id, cancel, &src_path, &tgt_path, policy, |_, _| {}).await?;
            }

            progress(tgt_path);
        }
    }

    Ok(dest_root)
}

/* =============================
   MOVE / RENAME (ATOMIC)
============================= */

pub async fn move_path(
    from: &Path,
    to: &Path,
    policy: ConflictPolicy,
) -> Result<PathBuf, FsError> {

    let dest = match resolve_path(to.to_path_buf(), policy).await {
        ConflictResolution::Use(p) => p,
        ConflictResolution::Skip => return Ok(to.to_path_buf()),
        ConflictResolution::Error(e) => return Err(FsError::Io(
            std::io::Error::new(std::io::ErrorKind::AlreadyExists, e)
        ))
    };

    match tokio::fs::rename(from, &dest).await {

        Ok(_) => Ok(dest),

        Err(_) => {
            // fallback
            tokio::fs::copy(from, &dest).await?;
            tokio::fs::remove_file(from).await?;
            Ok(dest)
        }
    }
}

/* =============================
   DELETE (CANCELLABLE)
============================= */

pub async fn delete_tree(
    id: JobId,
    cancel: &CancelRegistry,
    root: &Path,
) -> Result<(), FsError> {

    let mut stack = vec![root.to_path_buf()];
    let mut dirs = Vec::new(); // for post-order delete

    while let Some(path) = stack.pop() {

        if cancel.is_cancelled(id) {
            return Err(FsError::Cancelled(id));
        }

        let meta = fs::symlink_metadata(&path).await?;

        if meta.is_file() || meta.file_type().is_symlink() {
            fs::remove_file(&path).await?;
        } else {
            // directory → scan children first
            let mut rd = fs::read_dir(&path).await?;
            while let Some(entry) = rd.next_entry().await? {
                stack.push(entry.path());
            }
            dirs.push(path); // delete AFTER children
        }
    }

    // delete directories after contents
    for dir in dirs.into_iter().rev() {
        fs::remove_dir(dir).await?;
    }

    Ok(())
}


/* =============================
   MKDIR
============================= */

pub async fn create_dir(path: &Path) -> Result<(), FsError> {
    fs::create_dir_all(path).await?;
    Ok(())
}
