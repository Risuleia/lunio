use serde::{Deserialize, Serialize};

pub const PROTOCOL_VERSION: u8 = 1;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Request {
    Scan { root: String },
    Search { query: String, limit: Option<usize> },
    ListDir { path: String },

    RequestThumbnail { id: String },
    GetThumbnail { id: String },

    OpenFile { path: String },

    Shutdown
}

#[derive(Debug, Serialize)]
#[serde(tag = "status")]
pub enum Response {
    #[serde(rename = "ok")]
    Ok { data: Option<ResponseData> },

    #[serde(rename = "error")]
    Error { message: String },
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum ResponseData {
    SearchResults { entries: Vec<DaemonFileEntry> },
    DirectoryListing { entries: Vec<DaemonFileEntry> },
    Thumbnail { id: String, bytes: String },
    Ack
}

#[derive(Debug, Serialize)]
pub struct DaemonFileEntry {
    pub id: String,
    pub path: String,
    pub size: u64,
    pub is_dir: bool,
    pub modified: Option<i64>,
    pub has_thumbnail: bool
}

#[derive(Debug, Serialize)]
pub struct Handshake {
    pub protocol: u8,
    pub engine: String
}