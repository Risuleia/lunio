use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::{mpsc::Receiver, mpsc::Sender, RwLock};

use crate::{
    index::{
        event::IndexEvent,
        ingest::{self, initial_scan},
        model::{FileId, FileRecord},
        query::{self, Query},
        store::IndexStore,
    },
    watch::WatchEvent,
};

pub struct IndexService {
    store: Arc<RwLock<IndexStore>>,
    events: Sender<IndexEvent>,
}

impl IndexService {
    pub fn start(
        mut watch_rx: Receiver<WatchEvent>,
        events: Sender<IndexEvent>,
    ) -> Arc<Self> {

        let store = Arc::new(RwLock::new(IndexStore::new()));

        let svc = Arc::new(Self {
            store: store.clone(),
            events,
        });

        // Watcher ingestion loop
        let watcher_store = store.clone();
        let watcher_events = svc.events.clone();

        tokio::spawn(async move {
            while let Some(evt) = watch_rx.recv().await {
                let mut store = watcher_store.write().await;

                match evt {
                    WatchEvent::Created(path) | WatchEvent::Modified(path) => {
                        if let Some(prev) = store.get_by_path(&path).cloned() {
                            if let Some(new) = ingest::scan_file(&path, Some(&prev)) {
                                store.upsert(new.clone());

                                let _ = watcher_events.send(IndexEvent::Updated {
                                    id: new.id.clone(),
                                    path: new.path.clone(),
                                }).await;
                            }
                        } else if let Some(new) = ingest::scan_file(&path, None) {
                            store.upsert(new.clone());

                            let _ = watcher_events.send(IndexEvent::Inserted {
                                id: new.id.clone(),
                                path: new.path.clone(),
                            }).await;
                        }
                    }

                    WatchEvent::Deleted(path) => {
                        if let Some(old) = store.remove_by_path(&path) {
                            let _ = watcher_events.send(IndexEvent::Deleted {
                                id: old.id,
                                path,
                            }).await;
                        }
                    }
                }
            }
        });

        svc
    }

    /* ====================
       SCANNING
    ==================== */

    pub fn start_scan(&self, root: PathBuf) {
        let store = self.store.clone();
        let tx = self.events.clone();

        tokio::spawn(async move {
            initial_scan(root, store, tx).await;
        });
    }

    /* ====================
       QUERY API
    ==================== */

    pub async fn query(&self, q: Query) -> Vec<FileRecord> {
        let store = self.store.read().await;
        query::execute(&store, q)
    }

    /* ====================
       ADMIN API
    ==================== */

    pub async fn get(&self, id: &FileId) -> Option<FileRecord> {
        let store = self.store.read().await;
        store.get_by_id(id).cloned()
    }

    pub async fn count(&self) -> usize {
        let store = self.store.read().await;
        store.by_id.len()
    }
}
