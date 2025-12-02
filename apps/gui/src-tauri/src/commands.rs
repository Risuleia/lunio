use lunio_client::FileEntry;

use crate::client::{connect, with_client};

#[tauri::command]
pub fn cmd_connect() -> Result<(), String> {
    connect().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn cmd_search(query: String, limit: Option<usize>) -> Result<Vec<FileEntry>, String> {
    with_client(|c| {
        crate::client::RUNTIME
            .block_on(c.search(query, limit))
    }).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn cmd_list_dir(path: String) -> Result<Vec<FileEntry>, String> {
    with_client(|c| {
        crate::client::RUNTIME
            .block_on(c.list_dir(path))
    }).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn cmd_request_thumbnail(id: String) -> Result<(), String> {
    with_client(|c| {
        crate::client::RUNTIME
            .block_on(c.request_thumbnail(id))
    }).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn cmd_get_thumbnail(id: String) -> Result<Vec<u8>, String> {
    with_client(|c| {
        crate::client::RUNTIME
            .block_on(c.get_thumbnail(id))
    }).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn cmd_shutdown() -> Result<(), String> {
    with_client(|c| {
        crate::client::RUNTIME
            .block_on(c.shutdown())
    }).map_err(|e| e.to_string())
}