use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Error {
    pub code: ErrorCode,
    pub message: String,
    pub context: Option<HashMap<String, String>>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ErrorCode {
    NotFound,
    PermissionDenied,
    IoError,
    InvalidCommand,
    Timeout,
    IndexCorrupt,
    Unsupported,
    Internal,
}