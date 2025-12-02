use std::{path::Path, sync::Arc};

use lunio_core::EngineRuntime;

use crate::protocol::{DaemonFileEntry, Response, ResponseData};

pub async fn handle_list_dir(engine: Arc<EngineRuntime>, path: String) -> Response {
    let path = Path::new(&path);

    let entries = engine.list_dir(path);

    let out: Vec<DaemonFileEntry> = entries.into_iter()
        .map(|m| DaemonFileEntry {
            id: format!("{:032x}", m.id.0),
            path: m.path.to_string_lossy().into_owned(),
            size: m.size,
            is_dir: matches!(m.kind, lunio_core::models::FileKind::Directory),
            modified: m.modified
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs() as i64),
            has_thumbnail: m.has_thumbnail
        })
        .collect();
    
    Response::Ok { data: Some(ResponseData::DirectoryListing { entries: out }) }
}