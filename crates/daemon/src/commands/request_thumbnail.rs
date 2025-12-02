use std::sync::Arc;

use lunio_core::EngineRuntime;

use crate::protocol::{Response, ResponseData};

pub async fn handle_request_thumbnail(engine: Arc<EngineRuntime>, id_hex: String) -> Response {
    let id = match u128::from_str_radix(&id_hex, 16) {
        Ok(v ) => lunio_core::models::FileId(v),
        Err(_) => return Response::Error { message: "Invalid file id".into() }
    };

    if engine.request_thumbnail(id) {
        Response::Ok { data: Some(ResponseData::Ack) }
    } else {
        Response::Error { message: "file not found in index".into() }
    }
}