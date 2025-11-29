use std::{io::Cursor, path::Path};

use image::{DynamicImage, GenericImageView, imageops::FilterType};

use crate::thumbnails::generator::{ThumbnailError, ThumbnailResult};

pub fn is_supposed_image(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()).map(|s| s.to_lowercase()) {
        Some(ext) => matches!(
            ext.as_str(),
            "png" | "jpg" | "jpeg" | "webp" | "bmp" | "gif" | "tiff" | "tif"
        ),
        None => false
    }
}

pub fn generate_image_thumbnail(path: &Path, max_size: u32) -> ThumbnailResult<Vec<u8>> {
    let img = image::open(path)
        .map_err(|e| ThumbnailError::Image(e))?;

    let resized = resize_to_max(&img, max_size);

    let mut buf = Cursor::new(Vec::new());
    resized
        .write_to(&mut buf, image::ImageFormat::WebP)
        .map_err(|e| ThumbnailError::Image(e))?;

    Ok(buf.into_inner())
}

fn resize_to_max(img: &DynamicImage, max_side: u32) -> DynamicImage {
    let (w, h) = img.dimensions();
    let longest = w.max(h);

    if longest <= max_side {
        return img.clone();
    }

    let ratio = max_side as f32 / longest as f32;
    let new_w = (w as f32 * ratio) as u32;
    let new_h = (h as f32 * ratio) as u32;

    img.resize_exact(new_w, new_h, FilterType::CatmullRom)
}