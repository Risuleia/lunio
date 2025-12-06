use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{ServerCapability, error::Error};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Event {
    // Lifecycle
    Welcome {
        session_id: Uuid,
        server_version: String,
        server_capabilities: Vec<ServerCapability>,
    },
    Disconnected,

    Incompatible {
        reason: String,
        supported_protocol: u16,
    },

    // Filesystem
    FolderEntries {
        folder_id: u64,
        entries: Vec<FileEntry>,
    },
    FileChanged {
        path: String,
        kind: ChangeKind,
    },

    // Indexing
    IndexProgress {
        percent: u8,
    },

    // Search
    SearchResults {
        hits: Vec<SearchHit>,
    },
    DirectoryListing {
        entries: Vec<SearchHit>,
    },

    // Thumbnails
    ThumbStarted {
        id: Uuid,
        path: String,
    },
    ThumbReady {
        id: Uuid,
        path: String,
        thumb_path: String,
    },
    ThumbFailed {
        id: Uuid,
        path: String,
        reason: String,
    },

    JobList {
        jobs: Vec<JobInfo>,
    },

    JobProgress {
        id: Uuid,
        done: u64,
        total: u64,
    },
    JobUpdate {
        id: Uuid,
        status: String,
        done: u64,
        total: u64,
    },
    JobFailed {
        id: Uuid,
        reason: String,
    },

    // Error
    Error(Error),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileEntry {
    pub id: u64,
    pub name: String,
    pub is_dir: bool,
    pub size: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChangeKind {
    Created,
    Modified,
    Deleted,
    Renamed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchHit {
    pub file_id: u64,
    pub path: String,
    pub score: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JobInfo {
    pub id: uuid::Uuid,
    pub status: String,
    pub started_at: Option<u64>,
    pub finished_at: Option<u64>,
}
