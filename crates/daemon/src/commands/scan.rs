use lunio_core::EngineRuntime;

use crate::protocol::Response;

pub async fn handle_scan(engine: EngineRuntime, root: String) -> Response {
    engine.full_scan(root);
    Response::Ok { results: None }
}