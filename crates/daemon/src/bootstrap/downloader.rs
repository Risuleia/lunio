use std::{fs::{File, rename}, io::Write, path::{Path, PathBuf}};

use anyhow::{Context, Result};

pub async fn download(url: &str, dest: &Path) -> Result<()> {
    let tmp_path = PathBuf::from(&format!(
        "{}.tmp",
        dest.to_string_lossy().to_string(),
    ));
    
    let res = reqwest::get(url).await
        .with_context(|| format!("download failed: {url}"))?;
    if !res.status().is_success() {
        anyhow::bail!("HTTP error {} for {}", res.status(), url);
    }

    let bytes = res.bytes().await?;
    let mut file = File::create(&tmp_path)?;
    
    file.write_all(&bytes)?;

    rename(&tmp_path, dest)
        .with_context(|| format!("Failed to rename temporary file to {}", dest.display()))?;

    Ok(())
}