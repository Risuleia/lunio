use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThumbId(pub Uuid);

impl ThumbId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThumbSource {
    Image(PathBuf),
    Video(PathBuf),
    Pdf(PathBuf),
    Unknown(PathBuf),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ThumbSpec {
    pub id: ThumbId,
    pub source: ThumbSource,
    pub size: u32,
    pub priority: u8,     // 0 = lowest, 255 = highest
}

#[derive(Debug, Clone)]
pub enum ThumbStatus {
    Queued,
    Running,
    Completed(PathBuf),
    Failed(String),
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct ThumbResult {
    pub id: ThumbId,
    pub status: ThumbStatus,
}
