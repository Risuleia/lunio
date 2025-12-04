use std::{path::Path, sync::Arc};

use lunio_core::EngineRuntime;

use crate::protocol::Response;

pub async fn handle_open_file(engine: Arc<EngineRuntime>, path: String) -> Response {
    match engine.open_file(Path::new(&path)) {
        Ok(_) => Response::Ok { data: Some(crate::protocol::ResponseData::Ack) },
        Err(e) => Response::Error { message: e.to_string() }
    }
}