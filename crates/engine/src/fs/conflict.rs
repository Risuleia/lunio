use std::path::{Path, PathBuf};
use tokio::fs;

/* ============================
   CONFLICT POLICY
============================ */

#[derive(Debug, Clone, Copy)]
pub enum ConflictPolicy {
    Overwrite,
    Skip,
    Rename,
    Error,
}

/* ============================
   CONFLICT RESULT TYPE
============================ */

pub enum ConflictResolution {
    Use(PathBuf),
    Skip,
    Error(String),
}

pub async fn resolve_path(
    dest: PathBuf,
    policy: ConflictPolicy,
) -> ConflictResolution {

    match fs::metadata(&dest).await {

        // ✅ Path does NOT exist → safe to use
        Ok(_) => match policy {

            ConflictPolicy::Overwrite => ConflictResolution::Use(dest),

            ConflictPolicy::Skip => ConflictResolution::Skip,

            ConflictPolicy::Rename => ConflictResolution::Use(generate_copy_name(dest).await),

            ConflictPolicy::Error => ConflictResolution::Error(
                format!("destination already exists: {}", dest.display())
            ),
        },

        // ✅ Does NOT exist
        Err(_) => ConflictResolution::Use(dest),
    }
}

async fn generate_copy_name(path: PathBuf) -> PathBuf {

    let dir = path.parent().map(Path::to_path_buf).unwrap();
    let stem = path.file_stem().unwrap().to_string_lossy().to_string();
    let ext = path.extension().map(|e| e.to_string_lossy().to_string());

    let mut n = 1;

    loop {
        let name = match &ext {
            Some(e) => format!("{} ({}){}.{}", stem, n, "", e),
            None => format!("{} ({})", stem, n),
        };

        let candidate = dir.join(name);

        if fs::metadata(&candidate).await.is_err() {
            return candidate;
        }

        n += 1;
    }
}