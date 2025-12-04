mod bootstrapper;
mod downloader;
mod extractor;
mod manifest;
mod verifier;

pub use bootstrapper::bootstrap;
pub use manifest::load_manifest;


#[derive(Debug)]
pub struct RuntimeState {
    pub ffmpeg: Option<std::path::PathBuf>,
    pub pdfium: Option<std::path::PathBuf>
}

fn platform_key() -> String {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    format!("{}-{}", os, arch)
}