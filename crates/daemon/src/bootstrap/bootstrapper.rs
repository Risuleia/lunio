use std::path::{Path, PathBuf};

use anyhow::{Result, anyhow};

use crate::bootstrap::{RuntimeState, downloader, extractor, manifest::{RuntimeManifest, ToolBinary}, verifier};

pub async fn bootstrap(
    runtime_root: PathBuf,
    manifest: RuntimeManifest
) -> Result<RuntimeState> {
    std::fs::create_dir_all(&runtime_root)?;

    let ffmpeg = match install_tool(
        runtime_root.join("ffmpeg"),
        manifest.ffmpeg.resolve_tool()?,
        None
    ).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("[bootstrap] ffmpeg failed: {e}");
            None
        }
    };
    
    #[cfg(windows)]
    let particular_dir = Some(Path::new("bin"));

    #[cfg(not(windows))]
    let particular_dir = Some(Path::new("lib"));

    let pdfium = match install_tool(
        runtime_root.join("pdfium"),
        manifest.pdfium.resolve_tool()?,
        particular_dir
    ).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("[bootstrap] pdfium failed: {e}");
            None
        }
    };

    Ok(RuntimeState { ffmpeg, pdfium })
}

async fn install_tool(base: PathBuf, tool: &ToolBinary, particular_dir: Option<&Path>) -> Result<Option<PathBuf>> {
    let bin = base.join(&tool.path);

    if bin.exists() {
        return Ok(Some(bin));
    }

    std::fs::create_dir_all(&base)?;

    let name = tool.url.split("/").last()
        .ok_or_else(|| anyhow!("failed to get download name"))?;
    let tmp = base.join(name);

    downloader::download(&tool.url, &tmp).await?;

    if !tool.sha256.is_empty() {
        verifier::verify(&tool.sha256, &tmp)?;
    }
    
    extractor::extract(&tmp, &base, particular_dir)?;

    std::fs::remove_file(tmp)?;

    let final_bin = base.join(&tool.path);
    Ok(final_bin.exists().then_some(final_bin))
}