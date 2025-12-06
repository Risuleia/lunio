use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum Topic {
    FileSystem,
    Indexer,
    Thumbnails,
    Search,
    Jobs,
    Errors,
}