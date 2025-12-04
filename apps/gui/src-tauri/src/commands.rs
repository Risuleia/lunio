use lunio_client::FileEntry;

use crate::client;

#[tauri::command(async)]
pub async fn cmd_connect() -> Result<(), String> {
    client::connect().await.map_err(|e| e.to_string())
}

#[tauri::command(async)]
pub async fn cmd_search(query: String, limit: Option<usize>) -> Result<Vec<FileEntry>, String> {
    client::search(query, limit).await.map_err(|e| e.to_string())
}

#[tauri::command(async)]
pub async fn cmd_list_dir(path: String) -> Result<Vec<FileEntry>, String> {
    client::list_dir(path).await.map_err(|e| e.to_string())
}

#[tauri::command(async)]
pub async fn cmd_request_thumbnail(id: String) -> Result<(), String> {
    client::request_thumbnail(id).await.map_err(|e| e.to_string())
}

#[tauri::command(async)]
pub async fn cmd_get_thumbnail(id: String) -> Result<Vec<u8>, String> {
    client::get_thumbnail(id).await.map_err(|e| e.to_string())
}

#[tauri::command(async)]
pub async fn cmd_open_file(path: String) -> Result<(), String> {
    client::open_file(path).await.map_err(|e| e.to_string())
}

#[tauri::command(async)]
pub async fn cmd_shutdown() -> Result<(), String> {
    client::shutdown().await.map_err(|e| e.to_string())
}