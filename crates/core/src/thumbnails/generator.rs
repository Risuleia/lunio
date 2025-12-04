use std::{io, path::PathBuf};

use thiserror::Error;

use crate::{models::{FileId, FileKind, FileMeta}, thumbnails::formats::{images::generate_image_thumbnail, pdf::generate_pdf_thumbnail, video::generate_video_thumbnail}};

#[derive(Debug, Clone)]
pub struct ThumbnailConfig {
    pub max_size: u32,
    pub disk_cache_root: PathBuf,
    pub max_mem_bytes: usize,
    pub ffmpeg: Option<PathBuf>,
    pub pdfium: Option<PathBuf>
}

impl ThumbnailConfig {
    pub fn new(
        disk_cache_root: PathBuf,
        ffmpeg: Option<PathBuf>,
        pdfium: Option<PathBuf>
    ) -> Self {
        Self {
            max_size: 256,
            disk_cache_root,
            max_mem_bytes: 5 * 1024 * 1024,
            ffmpeg,
            pdfium
        }
    }

    pub fn disk_path_for(&self, id: FileId) -> PathBuf {
        let FileId(raw) = id;
        self.disk_cache_root.join(format!("{raw:032x}.webp"))
    }
}

enum ThumbKind {
    Image,
    Pdf,
    Video,
    Unsupported
}

#[derive(Error, Debug)]
pub enum ThumbnailError {
    #[error("unsupported file type")]
    Unsupported,

    #[error("missing external tool: {0}")]
    MissingTool(&'static str),

    #[error("io error: {0}")]
    Io(#[from] io::Error),

    #[error("image error: {0}")]
    Image(#[from] image::ImageError),

    #[error("external tool failure: {0}")]
    External(&'static str),
}

pub type ThumbnailResult<T> = Result<T, ThumbnailError>;

pub fn generate_thumbnail(
    meta: &FileMeta,
    cfg: &ThumbnailConfig
) -> ThumbnailResult<Vec<u8>> {
    if matches!(meta.kind, FileKind::Directory) {
        return Err(ThumbnailError::Unsupported);
    }

    let path = &meta.path;

    let kind = classify(path);
    match kind {
        ThumbKind::Image => {
            generate_image_thumbnail(
                &meta.path,
                cfg.max_size
            )
        }

        ThumbKind::Pdf => {
            let pdfium = cfg.pdfium
                .as_ref()
                .ok_or(ThumbnailError::MissingTool("pdfium"))?;

            generate_pdf_thumbnail(
                &meta.path,
                pdfium,
                cfg.max_size
            )
        }

        ThumbKind::Video => {
            let ffmpeg = cfg.ffmpeg
                .as_ref()
                .ok_or(ThumbnailError::MissingTool("ffmpeg"))?;

            generate_video_thumbnail(
                &meta.path,
                ffmpeg,
                cfg.max_size
            )
        }

        ThumbKind::Unsupported => Err(ThumbnailError::Unsupported)
    }
}

fn classify(path: &PathBuf) -> ThumbKind {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase());

    match ext.as_deref() {
        Some("png" | "jpg" | "jpeg" | "webp" | "bmp" | "gif" | "tiff" | "tif") => ThumbKind::Image,
        Some("pdf") => ThumbKind::Pdf,
        Some("mp4" | "mkv" | "mov" | "avi" | "webm" | "flv" | "wmv") => ThumbKind::Video,
        _ => ThumbKind::Unsupported
    }
}