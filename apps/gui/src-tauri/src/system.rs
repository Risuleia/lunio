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
    let mut out: Vec<SystemEntry> = Vec::new();

    push_folder(&mut out, "Desktop", dirs::desktop_dir().unwrap(), "desktop_windows");
    push_folder(&mut out, "Documents", dirs::document_dir().unwrap(), "description");
    push_folder(&mut out, "Downloads", dirs::download_dir().unwrap(), "download");
    push_folder(&mut out, "Pictures", dirs::picture_dir().unwrap(), "image");
    push_folder(&mut out, "Music", dirs::audio_dir().unwrap(), "music_note");
    push_folder(&mut out, "Videos", dirs::video_dir().unwrap(), "videocam");

    #[cfg(windows)]
    {
        #[link(name = "kernel32")]
            unsafe extern "system" {
                pub fn GetVolumeInformationW(
                    lpRootPathName: *const u16,
                    lpVolumeNameBuffer: *mut u16,
                    nVolumeNameSize: u32,
                    lpVolumeSerialNumber: *mut u32,
                    lpMaximumComponentLength: *mut u32,
                    lpFileSystemFlags: *mut u32,
                    lpFileSystemNameBuffer: *mut u16,
                    nFileSystemNameSize: u32,
                ) -> i32;
            }

            fn wchars_to_string(buf: &[u16]) -> String {
                use std::{ffi::OsString, os::windows::ffi::OsStringExt};

                let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
                OsString::from_wide(&buf[..len]).to_string_lossy().into_owned()
            }

            for letter in b'A'..=b'Z' {
                let drive = format!("{}:\\", letter as char);
                let path = std::path::Path::new(&drive);

                if path.exists() {
                    let wide_path: Vec<u16> = drive.encode_utf16().chain(std::iter::once(0)).collect();

                    let mut volume_name_buf: [u16; 256] = [0; 256];
                    let buf_size = volume_name_buf.len() as u32;

                    let success = unsafe {
                        GetVolumeInformationW(
                            wide_path.as_ptr(),
                            volume_name_buf.as_mut_ptr(),
                            buf_size,
                            std::ptr::null_mut(),
                            std::ptr::null_mut(),
                            std::ptr::null_mut(),
                            std::ptr::null_mut(),
                            0
                        )
                    };

                    let volume_label = if success != 0 {
                        let label = wchars_to_string(&volume_name_buf);
                        if label.is_empty() {
                            format!("Local Drive ({})", letter as char)
                        } else {
                            format!("{} ({})", label, letter as char)
                        }
                    } else {
                        format!("Local Drive ({})", letter as char)
                    };

                    out.push(SystemEntry {
                        label: volume_label,
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