use std::{fs, path::Path};

use crate::{fs::id::generate_file_id, models::FileMeta};

pub fn read_metadata(path: &Path) -> Option<FileMeta> {
    let meta = fs::metadata(path).ok()?;
    let id = generate_file_id(path);

    Some(FileMeta {
        version: 1,
        id,
        path: path.to_path_buf(),
        size: meta.len(),
        kind: if meta.is_dir() {
            crate::models::FileKind::Directory
        } else {
            crate::models::FileKind::File
        },
        modified: meta.modified().ok(),
        created: meta.created().ok(),
        has_thumbnail: false
    })
}