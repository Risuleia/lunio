use std::{io, path::PathBuf};

use thiserror::Error;

use crate::{models::{FileId, FileMeta}, thumbnails::formats::images::{generate_image_thumbnail, is_supposed_image}};

#[derive(Debug, Clone)]
pub struct ThumbnailConfig {
    pub max_size: u32,
    pub disk_cache_root: PathBuf
}

impl ThumbnailConfig {
    pub fn new(disk_cache_root: PathBuf) -> Self {
        Self {
            max_size: 256,
            disk_cache_root
        }
    }

    pub fn disk_path_for(&self, id: FileId) -> PathBuf {
        let FileId(raw) = id;
        self.disk_cache_root.join(format!("{raw:x}.webp"))
    }
}

#[derive(Error, Debug)]
pub enum ThumbnailError {
    #[error("unsupported file type")]
    Unsupported,

    #[error("io error: {0}")]
    Io(#[from] io::Error),

    #[error("image error: {0}")]
    Image(#[from] image::ImageError)
}

pub type ThumbnailResult<T> = Result<T, ThumbnailError>;

pub fn generate_thumbnail(
    meta: &FileMeta,
    cfg: &ThumbnailConfig
) -> ThumbnailResult<Vec<u8>> {
    if meta.is_dir {
        return Err(ThumbnailError::Unsupported);
    }

    let path = &meta.path;

    if is_supposed_image(path) {
        return Ok(generate_image_thumbnail(path, cfg.max_size)?);
    }

    Err(ThumbnailError::Unsupported)
}