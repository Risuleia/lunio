use std::{fs, sync::Arc};

use dashmap::DashMap;

use crate::{models::FileId, thumbnails::generator::{ThumbnailConfig, ThumbnailResult}};

pub struct ThumbnailCache {
    mem: DashMap<FileId, Arc<[u8]>>,
    pub cfg: ThumbnailConfig
}

impl ThumbnailCache {
    pub fn new(cfg: ThumbnailConfig) -> Self {
        fs::create_dir_all(&cfg.disk_cache_root).ok();

        Self {
            mem: DashMap::new(),
            cfg
        }
    }

    pub fn get(&self, id: FileId) -> Option<Arc<[u8]>> {
        if let Some(v) = self.mem.get(&id) {
            return Some(v.clone());
        }

        let disk = self.cfg.disk_path_for(id);
        if disk.exists() {
            if let Ok(bytes) = fs::read(&disk) {
                let arc: Arc<[u8]> = Arc::from(bytes.into_boxed_slice());
                self.mem.insert(id, arc.clone());
                return Some(arc);
            }
        }

        None
    }

    pub fn store(&self, id: FileId, bytes: &[u8]) -> ThumbnailResult<()> {
        let disk = self.cfg.disk_path_for(id);

        if let Some(parent) = disk.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&disk, bytes)?;

        let arc = Arc::from(bytes.to_vec().into_boxed_slice());
        self.mem.insert(id, arc);

        Ok(())
    }
}