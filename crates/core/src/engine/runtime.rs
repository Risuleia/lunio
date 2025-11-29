use std::{path::{Path, PathBuf}, sync::{Arc, RwLock}, thread};

use crate::{fs::{scan::scan_root, watcher::{FsChange, start_watcher}}, index::index::SimpleIndex, models::{FileId, FileMeta}, thumbnails::{cache::ThumbnailCache, generator::ThumbnailConfig, worker::ThumbnailWorker}};

#[derive(Clone)]
pub struct EngineRuntime {
    index: Arc<RwLock<SimpleIndex>>,
    thumb_cache: Arc<ThumbnailCache>,
    thumb_worker: ThumbnailWorker
}

impl EngineRuntime {
    pub fn new(cache_root: PathBuf) -> Self {
        let cfg = ThumbnailConfig::new(cache_root.clone());
        let cache = Arc::new(ThumbnailCache::new(cfg));
        let worker = ThumbnailWorker::new(cache.clone());

        Self {
            index: Arc::new(RwLock::new(SimpleIndex::new())),
            thumb_cache: cache,
            thumb_worker: worker,
        }
    }

    pub fn full_scan(&self, root: impl AsRef<Path>) {
        let metas = scan_root(root);
        
        let mut idx = self.index.write().unwrap();
        idx.apply_full_scan(metas);
    }

    pub fn start_watcher_loop(&self, root: impl AsRef<Path> + Clone + Send + Sync + 'static) {
        let root_path = root.as_ref().to_path_buf();
        let rx = start_watcher(root_path).expect("failed to start watcher");

        let index = self.index.clone();
        let worker = self.thumb_worker.clone();

        thread::spawn(move || {
            while let Ok(change) = rx.recv() {
                match change {
                    FsChange::Created(id, meta) |
                    FsChange::Modified(id, meta) => {
                        {
                            let mut idx = index.write().unwrap();
                            idx.apply_change(id, Some(meta.clone()));
                        }

                        worker.submit(meta);
                    }
                    FsChange::Deleted(id) => {
                        let mut idx = index.write().unwrap();
                        idx.apply_change(id, None);
                    }
                }
            }
        });
    }

    pub fn search(&self, query: &str, limit: usize) -> Vec<FileMeta> {
        let idx = self.index.read().unwrap();
        idx.search(query, limit)
    }

    pub fn get_thumbnail(&self, id: FileId) -> Option<Vec<u8>> {
        self.thumb_cache
            .get(id)
            .map(|arc| arc.to_vec())
    }

    pub fn request_thumbnail(&self, id: FileId) -> bool {
        let meta_opt = {
            let idx = self.index.read().unwrap();
            idx.files.get(&id).cloned()
        };

        if let Some(meta) = meta_opt {
            self.thumb_worker.submit(meta);
            true
        } else {
            false
        }
    }
}