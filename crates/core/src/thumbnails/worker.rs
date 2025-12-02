use std::{collections::HashSet, sync::{Arc, atomic::{AtomicBool, Ordering}, mpsc::{Receiver, SyncSender, sync_channel}}, thread::{self, JoinHandle}};

use parking_lot::RwLock;

use crate::{index::index::SimpleIndex, models::{FileId, FileMeta}, thumbnails::{cache::ThumbnailCache, generator::generate_thumbnail}};

#[derive(Debug, Clone)]
pub struct ThumbnailWorker {
    tx: SyncSender<Option<FileMeta>>,
    stop: Arc<AtomicBool>,
    _handle: Arc<JoinHandle<()>>
}

impl ThumbnailWorker {
    pub fn new(
        cache: Arc<ThumbnailCache>,
        index: Arc<RwLock<SimpleIndex>>
    ) -> Self {
        let (tx, rx) = sync_channel::<Option<FileMeta>>(64);
        let stop_flag = Arc::new(AtomicBool::new(false));

        let handle = {
            let stop = stop_flag.clone();
            let cache = cache.clone();
            let index = index.clone();

            thread::spawn(move || {
                println!("[thumb-worker] started");
                worker_loop(rx, cache, index, stop)
            })
        };

        Self { tx, stop: stop_flag, _handle: Arc::new(handle) }
    }

    pub fn submit(&self, meta: FileMeta) {
        let _ = self.tx.send(Some(meta));
    }

    pub fn shutdown(&self) {
        self.stop.store(true, Ordering::Relaxed);
        let _ = self.tx.send(None);
    }
}

fn worker_loop(
    rx: Receiver<Option<FileMeta>>,
    cache: Arc<ThumbnailCache>,
    index: Arc<RwLock<SimpleIndex>>,
    stop: Arc<AtomicBool>
) {
    let mut inflight: HashSet<FileId> = HashSet::new();

    while !stop.load(Ordering::Relaxed) {
        let Some(meta) = rx.recv().ok().flatten() else { break };

        let id = meta.id;
        
        if cache.get(id).is_some() {
            continue;
        }

        if inflight.contains(&id) {
            continue;
        }

        inflight.insert(id);

        match generate_thumbnail(&meta, &cache.cfg) {
            Ok(bytes) => {
                println!("[thumb-worker] generated {:?}", meta.path);
                let _ = cache.store(id, &bytes);

                // ✅ Update index: thumbnail now exists
                if let Some(m) = index.write().files.get_mut(&id) {
                    m.has_thumbnail = true;
                }
            }
            Err(e) => {
                println!("[thumb-worker] FAILED {:?} -> {:?}", meta.path, e);
            }
        }

        inflight.remove(&id);
    }
}