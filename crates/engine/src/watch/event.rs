use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum WatchEvent {
    Created(PathBuf),
    Modified(PathBuf),
    Deleted(PathBuf),
    // Renamed {
    //     from: PathBuf,
    //     to: PathBuf
    // }
}