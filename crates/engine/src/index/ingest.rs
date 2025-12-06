use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::UNIX_EPOCH,
};

use tokio::{fs, sync::{mpsc::Sender, RwLock}};

use crate::index::{
    event::IndexEvent,
    model::{FileId, FileRecord},
    store::IndexStore,
};
use crate::watch::WatchEvent;

/* ==========================
   PUBLIC ENTRY POINT
========================== */

pub fn ingest_event(store: &mut IndexStore, evt: WatchEvent) {
    match evt {
        WatchEvent::Created(path) | WatchEvent::Modified(path) => {
            // Best-effort: we don't pass prev here, service handles prev for watcher path
            if let Some(rec) = scan_file(&path, None) {
                store.upsert(rec);
            }
        }
        WatchEvent::Deleted(path) => {
            store.remove_by_path(&path);
        }
    }
}

/* ==========================
   INITIAL FULL SCAN
========================== */

pub async fn initial_scan(
    root: PathBuf,
    store: Arc<RwLock<IndexStore>>,
    events: Sender<IndexEvent>,
) {
    let _ = events.send(IndexEvent::Started {
        root: root.display().to_string(),
    }).await;

    let mut scanned: u64 = 0;
    let mut stack = vec![root];

    while let Some(path) = stack.pop() {

        let meta = match fs::metadata(&path).await {
            Ok(m) => m,
            Err(_) => continue,
        };

        if let Some(rec) = scan_file(&path, None) {
            let mut guard = store.write().await;
            guard.upsert(rec);
            scanned += 1;
        }

        if meta.is_dir() {
            if let Ok(mut rd) = fs::read_dir(&path).await {
                while let Ok(Some(e)) = rd.next_entry().await {
                    stack.push(e.path());
                }
            }
        }

        if scanned % 500 == 0 {
            let _ = events.send(IndexEvent::Progress {
                scanned,
                total: None,
            }).await;
        }
    }

    let _ = events.send(IndexEvent::Completed).await;
}

/* ==========================
   FILE INGEST LOGIC
========================== */

pub fn scan_file(path: &Path, prev: Option<&FileRecord>) -> Option<FileRecord> {
    use std::fs::symlink_metadata;

    let meta = symlink_metadata(path).ok()?;
    let abs = path.canonicalize().ok()?;

    let modified_unix = meta.modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let name = abs.file_name()?.to_string_lossy().to_string();

    let ext = abs.extension()
        .map(|x| x.to_string_lossy().to_lowercase());

    let parent = abs.parent()?.to_path_buf();

    Some(FileRecord {
        id: prev.map(|p| p.id.clone()).unwrap_or_else(FileId::new),

        path: abs,
        parent,
        name,
        ext,

        is_dir: meta.is_dir(),
        is_symlink: meta.file_type().is_symlink(),

        size: meta.len(),
        modified_unix,

        generation: prev.map(|p| p.generation + 1).unwrap_or(0),
    })
}
