use std::path::Path;

use crate::models::FileId;

#[cfg(unix)]
#[inline]
pub fn generate_file_id(path: &Path) -> Option<FileId> {
    use std::{fs, os::unix::fs::MetadataExt};
    let meta = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return None
    };

    let ino = meta.ino() as u128;
    let dev = meta.dev() as u128;

    Some(FileId((dev << 64) | ino))
}

#[cfg(windows)]
#[inline]
pub fn generate_file_id(path: &Path) -> Option<FileId> {
    use std::hash::{Hash, Hasher};
    use file_id::get_high_res_file_id;

    let fid = match get_high_res_file_id(path) {
        Ok(v) => v,
        Err(_) => return None
    };
    
    let mut hasher1 = ahash::AHasher::default();
    let mut hasher2 = ahash::AHasher::default();

    fid.hash(&mut hasher1);
    fid.hash(&mut hasher2);
    
    let h1 = hasher1.finish();
    let h2 = hasher2.finish().rotate_left(32).wrapping_mul(0x9E37_79B9_7F4A_7C15);

    Some(FileId(((h1 as u128) << 64) | (h2 as u128)))
}

#[cfg(not(any(unix, windows)))]
#[inline]
pub fn generate_file_id(path: &Path) -> Option<FileId> {
    use std::hash::{Hash, Hasher};

    let mut hasher = ahash::AHasher::default();
    path.hash(&mut hasher);
    Some(FileId(hasher.finish()))
}