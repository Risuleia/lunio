use std::{path::PathBuf, time::SystemTime};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FileId(pub u128);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileKind {
    File,
    Directory
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileMeta {
    pub version: u8,
    pub id: FileId,
    pub path: PathBuf,
    pub size: u64,
    pub kind: FileKind,
    pub modified: Option<SystemTime>,
    pub created: Option<SystemTime>,
    pub has_thumbnail: bool
}