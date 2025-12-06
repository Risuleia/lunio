use std::io::ErrorKind as IoKind;
use std::path::PathBuf;

#[derive(Debug)]
pub enum FsErrorKind {
    NotFound,
    PermissionDenied,
    AlreadyExists,
    IsDirectory,
    NotDirectory,
    InvalidName,
    ReadOnlyVolume,
    Cancelled,
    Unknown,
}

#[derive(Debug)]
pub struct FsError {
    pub kind: FsErrorKind,
    pub message: String,
    pub path: Option<PathBuf>,
}

impl FsError {
    pub fn new(kind: FsErrorKind, message: impl Into<String>, path: Option<PathBuf>) -> Self {
        Self {
            kind,
            message: message.into(),
            path,
        }
    }
}


pub fn map_io(err: std::io::Error, path: Option<PathBuf>) -> FsError {
    let kind = match err.kind() {
        IoKind::NotFound => FsErrorKind::NotFound,
        IoKind::PermissionDenied => FsErrorKind::PermissionDenied,
        IoKind::AlreadyExists => FsErrorKind::AlreadyExists,
        IoKind::InvalidInput => FsErrorKind::InvalidName,
        IoKind::ReadOnlyFilesystem => FsErrorKind::ReadOnlyVolume,
        _ => FsErrorKind::Unknown,
    };

    FsError::new(kind, err.to_string(), path)
}