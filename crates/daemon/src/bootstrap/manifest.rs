use std::collections::HashMap;

use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};

use crate::bootstrap::platform_key;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeManifest {
    pub version: u8,
    pub generated_at: String,
    pub ffmpeg: ToolEntry,
    pub pdfium: ToolEntry
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToolEntry {
    pub required: bool,
    pub platforms: HashMap<String, ToolBinary>
}

impl ToolEntry {
    pub fn resolve_tool<'a>(&'a self) -> Result<&'a ToolBinary> {
        let key = platform_key();

        let bin = self.platforms.get(&key)
            .ok_or_else(|| anyhow!("no build for `{key}`"))?;

        Ok(bin)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolBinary {
    pub version: String,
    pub url: String,
    pub sha256: String,
    pub path: String
}

pub async fn load_manifest() -> Result<RuntimeManifest> {
    let bundled = include_bytes!("../../assets/manifest.json");
    let manifest: RuntimeManifest = serde_json::from_slice(bundled)
        .context("bad embedded manifest")?;

    Ok(manifest)
}