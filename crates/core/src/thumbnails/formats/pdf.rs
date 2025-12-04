use std::path::Path;

use image::DynamicImage;
use pdfium_render::prelude::*;

use crate::thumbnails::generator::{ThumbnailError, ThumbnailResult};

pub fn generate_pdf_thumbnail(
    path: &Path,
    pdfium_path: &Path,
    max_size: u32
) -> ThumbnailResult<Vec<u8>> {
    let bindings = Pdfium::bind_to_library(pdfium_path)
        .map_err(|_| ThumbnailError::External("failed to load pdfium"))?;

    let pdfium = Pdfium::new(bindings);

    let doc = pdfium
        .load_pdf_from_file(path, None)
        .map_err(|_| ThumbnailError::External("failed to load pdf"))?;

    let page = doc.pages()
        .get(0)
        .map_err(|_| ThumbnailError::External("empty pdf"))?;

    let (w, h) = (page.width().value, page.height().value);

    let scale = compute_scale(w, h, max_size);

    let bitmap = page
        .render_with_config(
            &PdfRenderConfig::new()
                .set_target_width(((w * scale) as u16).into())
                .set_maximum_height(((h * scale) as u16).into())
                .render_form_data(true)
                .rotate_if_landscape(PdfPageRenderRotation::Degrees90, true)
        )
        .map_err(|_| ThumbnailError::External("pdf render failed"))?
        .as_image();

    encode_webp(bitmap)
}

fn compute_scale(w: f32, h: f32, max: u32) -> f32 {
    let longest = w.max(h);
    if longest <= max as f32 {
        1.0
    } else {
        max as f32 / longest
    }
}

fn encode_webp(img: DynamicImage) -> ThumbnailResult<Vec<u8>> {
    let mut buf = Vec::new();

    img.write_to(
        &mut std::io::Cursor::new(&mut buf),
        image::ImageFormat::WebP
    )
    .map_err(ThumbnailError::Image)?;

    Ok(buf)
}