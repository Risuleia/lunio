use std::{path::Path};

use rayon::iter::{ParallelBridge, ParallelIterator};
use walkdir::{DirEntry, WalkDir};

use crate::models::{FileId, FileMeta};

#[inline]
fn is_valid(entry: &DirEntry) -> bool {
    if entry.file_type().is_symlink() {
        return false
    }

    true
}

#[inline]
fn generate_file_id(path: &Path) -> FileId {
    use std::hash::{Hash, Hasher};

    let mut hasher = ahash::AHasher::default();
    path.hash(&mut hasher);

    FileId(hasher.finish())
}

pub fn scan_root<P: AsRef<Path>>(root: P) -> Vec<FileMeta> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| is_valid(e))
        .par_bridge()
        .filter_map(|entry| {
            let path = entry.path().to_path_buf();
            let metadata = entry.metadata().ok()?;

            Some(FileMeta {
                id: generate_file_id(entry.path()),
                path,
                size: metadata.len(),
                is_dir: metadata.is_dir(),
                modified: metadata.modified().ok(),
                created: metadata.created().ok(),
                has_thumbnail: false
            })
        })
        .collect()
}