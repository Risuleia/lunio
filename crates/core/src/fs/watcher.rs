use std::{path::PathBuf, sync::mpsc::{Receiver, channel}, time::Duration};
use notify::{Config, RecommendedWatcher, RecursiveMode, Result as NotifyResult, Watcher, Event};

use crate::{fs::metadata::{file_id_from_path, read_metadata}, models::{FileId, FileMeta}};

#[derive(Debug, Clone)]
pub enum FsChange {
    Created(FileId, FileMeta),
    Modified(FileId, FileMeta),
    Deleted(FileId)
}

pub fn start_watcher(root: PathBuf) -> NotifyResult<Receiver<FsChange>> {
    let (tx, rx) = channel::<FsChange>();

    let callback = move |res: NotifyResult<Event>| {
        match res {
            Ok(event) => {
                for path in event.paths {
                    let id = file_id_from_path(&path);

                    if event.kind.is_remove() {
                        let _ = tx.send(FsChange::Deleted(id));
                        continue;
                    }

                    if let Some(meta) = read_metadata(&path) {
                        if event.kind.is_create() {
                            let _ = tx.send(FsChange::Created(id, meta));
                        } else if event.kind.is_modify() {
                            let _ = tx.send(FsChange::Modified(id, meta));
                        }
                    }
                }
            },
            Err(err) => {
                eprintln!("[watcher] Error: {:?}", err);
            }
        }
    };

    let mut watcher = RecommendedWatcher::new(
        callback,
        Config::default()
            .with_poll_interval(Duration::from_secs(2))
            .with_compare_contents(true)
    )?;

    watcher.watch(&root, RecursiveMode::Recursive)?;

    Ok(rx)
}