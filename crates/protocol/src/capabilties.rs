use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ClientCapability {
    Thumbnails,
    LiveIndexing,
    Search,
    Reconnect,
    MultiWindow,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServerCapability {
    FullIndex,
    Thumbnails,
    FilesystemWatch,
    Search,
    Transactions,
}