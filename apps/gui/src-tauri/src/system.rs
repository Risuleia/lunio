use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SystemEntry {
    pub label: String,
    pub path: String,
    pub kind: String,
    pub icon: String
}

#[tauri::command]
pub fn get_sidebar_entries() -> Result<Vec<SystemEntry>, String> {
    let mut out = Vec::new();
    
    push_folder(&mut out, "Desktop", dirs::desktop_dir().unwrap(), "desktop_windows");
    push_folder(&mut out, "Documents", dirs::document_dir().unwrap(), "description");
    push_folder(&mut out, "Downloads", dirs::download_dir().unwrap(), "download");
    push_folder(&mut out, "Pictures", dirs::picture_dir().unwrap(), "image");
    push_folder(&mut out, "Music", dirs::audio_dir().unwrap(), "music_note");
    push_folder(&mut out, "Videos", dirs::video_dir().unwrap(), "videocam");

    
    #[cfg(windows)]
    {
        for letter in b'A'..=b'Z' {
            let drive = format!("{}:\\", letter as char);
            if std::path::Path::new(&drive).exists() {
                out.push(SystemEntry {
                    label: format!("Local Disk ({})", letter as char),
                    path: drive,
                    kind: "drive".into(),
                    icon: "hard_drive".into()
                });
            }
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        let vols = PathBuf::from("/Volumes");
        if let Ok(entries) = std::fs::read_dir(vols) {
            for e in entries.flatten() {
                out.push(SystemEntry {
                    label: e.file_name().to_string_lossy().into_owned(),
                    path: e.path().display().to_string(),
                    kind: "drive".into(),
                    icon: "hard_drive".into()
                });
            }
        }
    }

    
    #[cfg(target_os = "linux")]
    {
        // Root
        out.push(SystemEntry {
            label: "Root".into(),
            path: "/".into(),
            kind: "drive".into(),
        });

        collect_mounts(&mut out, "/mnt");
        collect_mounts(&mut out, "/media");
    }

    Ok(out)
}

fn push_folder(out: &mut Vec<SystemEntry>, label: &str, path: PathBuf, icon: &str) {
    if path.exists() {
        out.push(SystemEntry {
            label: label.into(),
            path: path.display().to_string(),
            kind: "folder".into(),
            icon: icon.into()
        });
    }
}

#[cfg(target_os = "linux")]
fn collect_mounts(out: &mut Vec<SystemEntry>, root: &str) {
    let base = PathBuf::from(root);
    if let Ok(entries) = std::fs::read_dir(base) {
        for e in entries.flatten() {
            out.push(SystemEntry {
                label: e.file_name().to_string_lossy().into_owned(),
                path: e.path().display().to_string(),
                kind: "drive".into(),
                icon: "hard_drive".into()
            });
        }
    }
}