use std::{fs, io, path::{Path, PathBuf}, process::Command, time::SystemTime};

use crate::thumbnails::generator::{ThumbnailError, ThumbnailResult};

pub fn generate_video_thumbnail(
    video: &Path,
    ffmpeg: &Path,
    max_size: u32
) -> ThumbnailResult<Vec<u8>> {
    let tmp = temp_webp_path()?;
    
    let scale = format!("scale='if(gt(iw,ih),{},-2)':'if(gt(iw,ih),-2,{})'", max_size, max_size);

    let status = Command::new(ffmpeg)
        .args([
            "-hide_banner",
            "-loglevel", "error",
            "-y",

            "-ss", "00:00:01",
            "-i", video.to_str().ok_or(ThumbnailError::Io(io_err("bad path")))?,

            "-vf", &format!("thumbnail,{}", scale),
            "-frames:v", "1",

            tmp.to_str().ok_or(ThumbnailError::Io(io_err("bad temp path")))?
        ])
        .status()
        .map_err(ThumbnailError::Io)?;

    if !status.success() {
        return Err(ThumbnailError::External("ffmpeg failed to extract frame"));
    }

    let bytes = fs::read(&tmp).map_err(ThumbnailError::Io)?;
    let _ = fs::remove_file(&tmp);

    Ok(bytes)
}

fn temp_webp_path() -> io::Result<PathBuf> {
    let mut p = std::env::temp_dir();
    let now = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    p.push(format!("lunio-thumb-{now}.webp"));
    Ok(p)
}

fn io_err(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, msg)
}