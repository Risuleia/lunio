use std::{fs, path::Path};

use anyhow::{Result, bail};
use sha2::{Digest, Sha256};

pub fn verify(expected: &str, file: &Path) -> Result<()> {
    let data = fs::read(file)?;
    let hash = Sha256::digest(&data);
    let actual = format!("{:x}", hash);

    if actual != expected {
        bail!("checksum mismatch for {} (expected {}, got {})",
            file.display(), expected, actual
        );
    }

    Ok(())
}