use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::{ClientCapability, topic::Topic};

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Hello {
        protocol: u16,
        client_id: Uuid,
        client_version: String,
        capabilities: Vec<ClientCapability>,
    },
    Disconnect,

    // Stream control
    Subscribe { topics: Vec<Topic> },

    // Filesystem
    OpenFolder { path: String },
    ReadDir { folder_id: u64 },
    Stat { path: String },
    Delete { path: String },
    Move { from: String, to: String },
    Copy { from: String, to: String },
    
    ListJobs,

    // Search
    Search { query: String },
    Browse { path: String },

    // Thumbnails
    RequestThumbnail { file_id: u64 },
}