#[cfg(target_os = "windows")]
use std::process::Command;
use std::{path::{Path, PathBuf}, sync::{Arc, atomic::{AtomicBool, Ordering}}, thread::{self, JoinHandle}};

use notify::RecommendedWatcher;
use parking_lot::RwLock;

use crate::{fs::{scan::scan_root, watcher::{FsChange, FsWatcher, start_watcher}}, index::index::SimpleIndex, models::{FileId, FileMeta}, thumbnails::{cache::ThumbnailCache, generator::ThumbnailConfig, worker::ThumbnailWorker}};

pub struct EngineRuntime {
    index: Arc<RwLock<SimpleIndex>>,
    thumb_cache: Arc<ThumbnailCache>,
    thumb_worker: ThumbnailWorker,
    stop_flag: Arc<AtomicBool>,
    watch_thread: Arc<RwLock<Option<JoinHandle<()>>>>,
    watcher: Arc<RwLock<Option<RecommendedWatcher>>>
}

impl EngineRuntime {
    pub fn new(
        cache_root: PathBuf,
        ffmpeg: Option<PathBuf>,
        pdfium: Option<PathBuf>
    ) -> Self {
        let cfg = ThumbnailConfig::new(cache_root.clone(), ffmpeg, pdfium);
        let cache = Arc::new(ThumbnailCache::new(cfg));
        
        let index = Arc::new(RwLock::new(SimpleIndex::new()));

        let worker = ThumbnailWorker::new(cache.clone(), index.clone());

        Self {
            index,
            thumb_cache: cache,
            thumb_worker: worker,
            stop_flag: Arc::new(AtomicBool::new(false)),
            watch_thread: Arc::new(RwLock::new(None)),
            watcher: Arc::new(RwLock::new(None))
        }
    }

    pub fn full_scan(&self, root: impl AsRef<Path>) {
        let metas = scan_root(root);
        
        self.index.write().apply_full_scan(metas);
    }

    pub fn start_watcher_loop(&mut self, root: impl AsRef<Path>) {
        let root = root.as_ref().to_path_buf();
        let FsWatcher { rx, watcher } = start_watcher(root).expect("failed to start watcher");
        *self.watcher.write() = Some(watcher);

        let index = self.index.clone();
        let worker = self.thumb_worker.clone();
        let stop = self.stop_flag.clone();

        let handle = thread::spawn(move || {
            while !stop.load(Ordering::Relaxed) {
                match rx.recv() {
                    Ok(change) => {
                        match change {
                            FsChange::Created(id, meta) |
                            FsChange::Modified(id, meta) => {
                                index.write().apply_change(id, Some(meta.clone()));
                                worker.submit(meta);
                            }
                            FsChange::Deleted(id) => {
                                index.write().apply_change(id, None);
                            }
                        }
                    },
                    Err(_) => break
                }
            }
        });

        *self.watch_thread.write() = Some(handle)
    }

    pub fn shutdown(&self) {
        self.stop_flag.store(true, Ordering::Relaxed);

        if let Some(handle) = self.watch_thread.write().take() {
            let _ = handle.join();
        }

        *self.watcher.write() = None;
        self.thumb_worker.shutdown();
    }

    pub fn search(&self, query: &str, limit: usize) -> Vec<FileMeta> {
        self.index.read().search(query, limit)
    }

    pub fn get_thumbnail(&self, id: FileId) -> Option<Vec<u8>> {
        self.thumb_cache
            .get(id)
            .map(|arc| arc.to_vec())
    }

    pub fn request_thumbnail(&self, id: FileId) -> bool {
        let meta = self.index.read().get(id).cloned();

        if let Some(meta) = meta {
            self.thumb_worker.submit(meta);
            true
        } else {
            false
        }
    }

    fn is_indexed(&self, path: &Path) -> bool {
        self.index.read().files.values().any(|m| m.path.starts_with(path))
    }

    pub fn list_dir(&self, path: &Path) -> Vec<FileMeta> {
        if !self.is_indexed(path) {
            self.full_scan(path);
        }

        let idx = self.index.read();
        let base = normalize(path);

        let mut out: Vec<_> = idx.files
            .values()
            .filter(|m| {
                m.path.parent()
                    .map(normalize)
                    .map(|p| p == base)
                    .unwrap_or(false)
            })
            .cloned()
            .collect();

        out.sort_by_key(|m| (!matches!(m.kind, crate::models::FileKind::Directory), m.path.clone()));
        out
    }

    pub fn open_file(&self, path: &Path) -> anyhow::Result<()> {
        let path = path.to_string_lossy().to_string();

        #[cfg(target_os = "windows")]
        Command::new("cmd")
            .args(["/C", "start", "", &path])
            .spawn()?;

        #[cfg(target_os = "macos")]
        Command::new("open")
            .arg(&path)
            .spawn()?;

        #[cfg(target_os = "linux")]
        Command::new("xdg-open")
            .arg(&path)
            .spawn()?;

        Ok(())
    }
}

fn normalize(p: &Path) -> PathBuf {
    p.components().collect()
}