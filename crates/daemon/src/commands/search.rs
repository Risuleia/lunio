use lunio_core::EngineRuntime;

use crate::protocol::{DaemonFileEntry, Response};

pub async fn handle_search(
    engine: EngineRuntime,
    query: String,
    limit: Option<usize>
) -> Response {
    let results = engine.search(&query, limit.unwrap_or(50));

    let mapped = results
        .into_iter()
        .map(|m| DaemonFileEntry {
            path: m.path.to_string_lossy().into_owned(),
            size: m.size
        })
        .collect::<Vec<_>>();

    Response::Ok { results: Some(mapped) }
}