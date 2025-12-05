use std::{path::PathBuf, sync::mpsc::{Receiver, channel}};
use notify::{Config, RecommendedWatcher, RecursiveMode, Result as NotifyResult, Watcher, Event};

use crate::{fs::{id::generate_file_id, metadata::read_metadata}, models::{FileId, FileMeta}};

#[derive(Debug, Clone)]
pub enum FsChange {
    Created(FileId, FileMeta),
    Modified(FileId, FileMeta),
    Deleted(FileId)
}

pub struct FsWatcher {
    pub rx: Receiver<FsChange>,
    pub watcher: RecommendedWatcher
}

pub fn start_watcher(root: PathBuf) -> NotifyResult<FsWatcher> {
    let (tx, rx) = channel::<FsChange>();

    let callback = move |res: NotifyResult<Event>| {
        let event = match res {
            Ok(e) => e,
            Err(err) => {
                eprintln!("[watcher] Error: {:?}", err);
                return;
            }
        };

        match event.kind {
            notify::EventKind::Create(_) |
            notify::EventKind::Modify(_) |
            notify::EventKind::Remove(_) => {},
            _ => return
        }

        for path in event.paths {
            if matches!(event.kind, notify::EventKind::Remove(_)) {
                let id = match generate_file_id(&path) {
                    Some(id) => id,
                    None => continue
                };
                let _ = tx.send(FsChange::Deleted(id));
                continue;
            }
            
            let id = match generate_file_id(&path) {
                Some(id) => id,
                None => continue
            };

            if let Some(meta) = read_metadata(&path) {
                if matches!(event.kind, notify::EventKind::Create(_)) {
                    let _ = tx.send(FsChange::Created(id, meta));
                } else {
                    let _ = tx.send(FsChange::Modified(id, meta));
                }
            }
        }
    };

    let mut watcher = RecommendedWatcher::new(callback, Config::default())?;
    watcher.watch(&root, RecursiveMode::Recursive)?;

    Ok(FsWatcher { rx, watcher })
}