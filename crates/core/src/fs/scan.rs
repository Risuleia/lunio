use std::{path::Path};
use rayon::iter::{ParallelBridge, ParallelIterator};
use walkdir::{DirEntry, WalkDir};

use crate::{METADTA_VERSION, fs::id::generate_file_id, models::FileMeta};

#[inline]
fn is_valid(entry: &DirEntry) -> bool {
    let ft = entry.file_type();
    ft.is_file() | ft.is_dir()
}

pub fn scan_root<P: AsRef<Path>>(root: P) -> Vec<FileMeta> {
    WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| is_valid(e))
        .par_bridge()
        .filter_map(|entry| {
            let path = entry.path().to_path_buf();
            let metadata = entry.metadata().ok()?;
            let id = generate_file_id(&path)?;

            Some(FileMeta {
                version: METADTA_VERSION,
                id,
                path,
                size: metadata.len(),
                kind: if metadata.is_dir() {
                    crate::models::FileKind::Directory
                } else {
                    crate::models::FileKind::File
                },
                modified: metadata.modified().ok(),
                created: metadata.created().ok(),
                has_thumbnail: false
            })
        })
        .collect()
}