use std::{path::PathBuf, time::SystemTime};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FileId(pub u64);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileMeta {
    pub id: FileId,
    pub path: PathBuf,
    pub size: u64,
    pub is_dir: bool,
    pub modified: Option<SystemTime>,
    pub created: Option<SystemTime>,
    pub has_thumbnail: bool
}