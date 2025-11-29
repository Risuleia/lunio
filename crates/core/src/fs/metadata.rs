use std::{fs, path::Path};

use crate::models::{FileId, FileMeta};

#[inline]
pub fn file_id_from_path(path: &Path) -> FileId {
    use std::hash::{Hash, Hasher};

    let mut hasher = ahash::AHasher::default();
    path.hash(&mut hasher);

    FileId(hasher.finish())
}

pub fn read_metadata(path: &Path) -> Option<FileMeta> {
    let meta = fs::metadata(path).ok()?;

    Some(FileMeta {
        id: file_id_from_path(path),
        path: path.to_path_buf(),
        size: meta.len(),
        is_dir: meta.is_dir(),
        modified: meta.modified().ok(),
        created: meta.created().ok(),
        has_thumbnail: false
    })
}