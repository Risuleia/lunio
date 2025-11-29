use std::{sync::{Arc, mpsc::{Receiver, Sender, channel}}, thread};

use crate::{models::FileMeta, thumbnails::{cache::ThumbnailCache, generator::generate_thumbnail}};

#[derive(Debug, Clone)]
pub struct ThumbnailJob {
    pub meta: FileMeta
}

#[derive(Debug, Clone)]
pub struct ThumbnailWorker {
    tx: Sender<ThumbnailJob>
}

impl ThumbnailWorker {
    pub fn new(cache: Arc<ThumbnailCache>) -> Self {
        let (tx, rx) = channel::<ThumbnailJob>();

        thread::spawn(move || worker_loop(rx, cache));

        Self { tx }
    }

    pub fn submit(&self, meta: FileMeta) {
        let _ = self.tx.send(ThumbnailJob { meta });
    }
}

fn worker_loop(rx: Receiver<ThumbnailJob>, cache: Arc<ThumbnailCache>) {
    while let Ok(job) = rx.recv() {
        let id = job.meta.id;

        if cache.get(id).is_some() {
            continue;
        }

        if let Ok(bytes) = generate_thumbnail(&job.meta, &cache.cfg) {
            let _ = cache.store(id, &bytes);
        }
    }
}