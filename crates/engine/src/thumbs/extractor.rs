use std::{path::PathBuf, process::Stdio, time::Duration};
use image::{DynamicImage, GenericImageView};
use pdfium_render::prelude::{PdfRenderConfig, Pdfium};
use tokio::{process::Command, time::timeout};

const EXEC_TIMEOUT: Duration = Duration::from_secs(10);

pub struct ExtractorEnv {
    pub ffmpeg: Option<PathBuf>,
    pub pdfium: Option<PathBuf>,
}

/* ================================
   VIDEO THUMBNAIL (NO FFPROBE)
================================ */

pub async fn extract_video(
    path: &PathBuf,
    size: u32,
    env: &ExtractorEnv,
) -> Result<Vec<u8>, String> {

    let ffmpeg = env.ffmpeg.as_ref().ok_or("ffmpeg not available")?;

    // Seek a small fixed offset to avoid black frames at t=0.
    // If the file is shorter or seek fails, ffmpeg will still attempt decode.
    let seek = "1"; // seconds
    let scale = format!("scale=-2:{}", size);

    let mut cmd = Command::new(ffmpeg);
    cmd.args([
        "-loglevel", "error",
        "-hide_banner",
        "-ss", seek,
        "-i", path.to_string_lossy().as_ref(),
        "-frames:v", "1",
        "-vf", &scale,
        "-f", "image2pipe",
        "-vcodec", "png",
        "pipe:1",
    ])
    .stdout(Stdio::piped())
    .stderr(Stdio::piped());

    let child = cmd.spawn().map_err(|e| format!("ffmpeg spawn failed: {e}"))?;

    let output = timeout(EXEC_TIMEOUT, child.wait_with_output())
        .await
        .map_err(|_| "ffmpeg timed out".to_string())?
        .map_err(|e| format!("ffmpeg failed: {e}"))?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(format!("ffmpeg error: {err}"));
    }

    if output.stdout.is_empty() {
        return Err("ffmpeg returned empty output".into());
    }

    Ok(output.stdout)
}

/* -----------------------------
   STUBS FOR OTHER FORMATS
------------------------------ */

pub async fn extract_image(
    path: &PathBuf,
    size: u32,
    _env: &ExtractorEnv,
) -> Result<Vec<u8>, String> {

    let path = path.clone();

    // Heavy work off the async runtime
    let img = tokio::task::spawn_blocking(move || {
        image::open(&path).map_err(|e| e.to_string())
    }).await.map_err(|e| e.to_string())??;

    let (w, h) = img.dimensions();
    if w == 0 || h == 0 {
        return Err("invalid image dimensions".into());
    }

    // Maintain aspect ratio
    let scale = (size as f32 / h as f32).min(size as f32 / w as f32);
    let new_w = (w as f32 * scale) as u32;
    let new_h = (h as f32 * scale) as u32;

    let resized = img.resize(new_w, new_h, image::imageops::FilterType::Lanczos3);

    // Encode PNG
    let mut buf = Vec::with_capacity((new_w * new_h * 4) as usize);
    resized
        .write_to(&mut std::io::Cursor::new(&mut buf), image::ImageFormat::Png)
        .map_err(|e| e.to_string())?;

    Ok(buf)
}

pub async fn extract_pdf(
    path: &PathBuf,
    size: u32,
    env: &ExtractorEnv,
) -> Result<Vec<u8>, String> {

    let pdf_path = path.clone();
    let pdfium_bin = env.pdfium.clone().ok_or("pdfium not available")?;

    // Heavy native work must be off the async executor
    let png = tokio::task::spawn_blocking(move || {
        // Bind to daemon-managed PDFium binary
        let bindings = Pdfium::bind_to_library(pdfium_bin)
            .map_err(|e| format!("pdfium bind failed: {e}"))?;

        let pdfium = Pdfium::new(bindings);

        let doc = pdfium.load_pdf_from_file(&pdf_path, None)
            .map_err(|e| format!("load pdf failed: {e}"))?;

        let page = doc.pages().get(0)
            .map_err(|_| "pdf has no pages".to_string())?;

        // Render with a max dimension constraint
        let render = page.render_with_config(
            &PdfRenderConfig::new()
                .set_maximum_width(size.try_into().unwrap())
                .rotate_if_landscape(pdfium_render::prelude::PdfPageRenderRotation::None, false)
                .render_form_data(true)
        )
        .map_err(|e| format!("pdf render failed: {e}"))?;

        // Convert PDFBitmap -> image::DynamicImage
        let bmp = render.as_image();

        let dyn_img = DynamicImage::ImageRgba8(bmp.into());

        // Encode PNG
        let mut out = Vec::new();
        dyn_img
            .write_to(&mut std::io::Cursor::new(&mut out), image::ImageFormat::Png)
            .map_err(|e| e.to_string())?;

        Ok::<Vec<u8>, String>(out)
    })
    .await
    .map_err(|e| e.to_string())??;

    Ok(png)
}
