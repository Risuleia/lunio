use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Request {
    Search { query: String, limit: Option<usize> },
    Scan { root: String },
    Shutdown
}

#[derive(Debug, Serialize)]
#[serde(tag = "status")]
pub enum Response {
    #[serde(rename = "ok")]
    Ok { results: Option<Vec<DaemonFileEntry>> },

    #[serde(rename = "error")]
    Error { message: String },
}

#[derive(Debug, Serialize)]
pub struct DaemonFileEntry {
    pub path: String,
    pub size: u64
}