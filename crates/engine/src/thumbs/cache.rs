use std::{path::{Path, PathBuf}, fs};
use sha2::{Sha256, Digest};

#[derive(Clone)]
pub struct ThumbCache {
    root: PathBuf,
}

impl ThumbCache {
    pub fn new(root: impl AsRef<Path>) -> Self {
        let s = Self { root: root.as_ref().to_path_buf() };
        fs::create_dir_all(&s.root).ok();
        s
    }

    pub fn cache_path(&self, key: &str) -> PathBuf {
        self.root.join(format!("{key}.png"))
    }

    pub fn exists(&self, key: &str) -> bool {
        self.cache_path(key).exists()
    }

    pub fn write_atomic(&self, key: &str, data: &[u8]) -> std::io::Result<PathBuf> {
        let final_path = self.cache_path(key);
        let tmp = final_path.with_extension("tmp");

        fs::write(&tmp, data)?;
        fs::rename(&tmp, &final_path)?;

        Ok(final_path)
    }

    pub fn hash_path(path: &Path, size: u32) -> String {
        let mut hasher = Sha256::new();
        hasher.update(path.to_string_lossy().as_bytes());
        hasher.update(&size.to_le_bytes());
        format!("{:x}", hasher.finalize())
    }
}
