use std::{collections::HashSet, fs::File, io::Read, path::{Path, PathBuf}};
use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use tar::Archive;
use xz2::read::XzDecoder;
use zip::ZipArchive;

pub fn extract(archive: &Path, target: &Path, particular_dir: Option<&Path>) -> Result<()> {
    std::fs::create_dir_all(target)?;

    if let Some(ext) = archive.extension().and_then(|e| e.to_str()) {
        match ext {
            "zip" => extract_zip(archive, target, particular_dir)?,
            "gz" | "tgz" => extract_tar_gz(archive, target, particular_dir)?,
            "xz" | "xgz" => extract_tar_xz(archive, target, particular_dir)?,
            _ => anyhow::bail!("unknown archive"),
        }
    }

    Ok(())
}

fn extract_zip(path: &Path, out: &Path, pd: Option<&Path>) -> Result<()> {
    let file = File::open(path)?;
    let mut zip = ZipArchive::new(file)?;

    let mut paths = Vec::new();
    for i in 0..zip.len() {
        paths.push(PathBuf::from(zip.by_index(i)?.name()));
    }

    let root = detect_common_root(&paths);

    for i in 0..zip.len() {
        let mut item = zip.by_index(i)?;
        let raw = Path::new(item.name());

        let stripped = strip_root(raw, root.as_deref());
        let final_path = match apply_particular_dir(stripped, pd, root.as_deref()) {
            Some(p) => p,
            None => continue
        };

        let dest = out.join(&final_path);

        if !dest.starts_with(out) {
            anyhow::bail!("malicious path traversal: {:?}", final_path);
        }

        if raw.as_os_str().to_string_lossy().ends_with('/') {
            std::fs::create_dir_all(&dest)?;
        } else {
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut out_file = File::create(dest)?;
            std::io::copy(&mut item, &mut out_file)?;
        }
    }

    Ok(())
}

fn extract_tar_archive<R: Read>(
    reader: R,
    out: &Path,
    pd: Option<&Path>,
) -> Result<()> {
    let mut archive = Archive::new(reader);

    let mut root: Option<PathBuf> = None;
    let mut root_confirmed = true;

    for entry_result in archive.entries()? {
        let mut entry = entry_result?;
        let raw = entry.path()?.into_owned();

        if let Some(first) = raw.components().next() {
            let first = PathBuf::from(first.as_os_str());

            match root.as_ref() {
                None => root = Some(first),
                Some(r) if r != &first => root_confirmed = false,
                _ => {}
            }
        }

        let stripped = if root_confirmed {
            if let Some(r) = &root {
                raw.strip_prefix(r).unwrap_or(&raw).to_path_buf()
            } else {
                raw.clone()
            }
        } else {
            raw.clone()
        };

        let final_path = match apply_particular_dir(stripped, pd, root.as_deref()) {
            Some(p) if !p.as_os_str().is_empty() => p,
            _ => continue,
        };

        let dest = out.join(&final_path);

        if !dest.starts_with(out) {
            anyhow::bail!("malicious path traversal: {:?}", final_path);
        }

        if entry.header().entry_type().is_dir() {
            std::fs::create_dir_all(&dest)?;
        } else {
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent)?;
            }
            entry.unpack(&dest)
                .with_context(|| format!("Failed to unpack {:?}", final_path))?;
        }
    }

    Ok(())
}

fn extract_tar_gz(path: &Path, out: &Path, pd: Option<&Path>) -> Result<()> {
    extract_tar_archive(GzDecoder::new(File::open(path)?), out, pd)
}

fn extract_tar_xz(path: &Path, out: &Path, pd: Option<&Path>) -> Result<()> {
    extract_tar_archive(XzDecoder::new(File::open(path)?), out, pd)
}

fn detect_common_root(paths: &[PathBuf]) -> Option<PathBuf> {
    let mut roots = HashSet::new();

    for p in paths {
        if let Some(first) = p.components().next() {
            roots.insert(first.as_os_str().to_os_string());
        }
    }

    if roots.len() == 1 {
        Some(PathBuf::from(roots.into_iter().next().unwrap()))
    } else {
        None
    }
}

fn strip_root(path: &Path, root: Option<&Path>) -> PathBuf {
    if let Some(root) = root {
        path.strip_prefix(root).unwrap_or(path).to_path_buf()
    } else {
        path.to_path_buf()
    }
}

fn normalize_particular_dir(pd: &Path, root: Option<&Path>) -> PathBuf {
    if let Some(root) = root {
        pd.strip_prefix(root).unwrap_or(pd).to_path_buf()
    } else {
        pd.to_path_buf()
    }
}

fn apply_particular_dir(path: PathBuf, pd: Option<&Path>, root: Option<&Path>) -> Option<PathBuf> {
    if let Some(pd) = pd {
        let norm = normalize_particular_dir(pd, root);

        if norm.as_os_str().is_empty() {
            return Some(path);
        }

        match path.strip_prefix(&norm) {
            Ok(sub) if !sub.as_os_str().is_empty() => Some(sub.to_path_buf()),
            _ => None
        }
    } else {
        Some(path)
    }
}