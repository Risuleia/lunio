use std::sync::Arc;

use lunio_core::EngineRuntime;

use crate::protocol::{Response, ResponseData};

pub async fn handle_scan(engine: Arc<EngineRuntime>, root: String) -> Response {
    if root.trim().is_empty() {
        return Response::Error { message: "Root path cannot be empty".into() };
    }

    engine.full_scan(root);
    Response::Ok { data: Some(ResponseData::Ack) }
}