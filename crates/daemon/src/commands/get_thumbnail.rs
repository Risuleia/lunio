use std::sync::Arc;

use base64::{Engine, engine::general_purpose};
use lunio_core::EngineRuntime;

use crate::protocol::{Response, ResponseData};

pub async fn handle_get_thumbnail(engine: Arc<EngineRuntime>, id_hex: String) -> Response {
    let id = match u128::from_str_radix(&id_hex, 16) {
        Ok(v) => lunio_core::models::FileId(v),
        Err(_) => return Response::Error { message: "Invalid file id".into() }
    };

    match engine.get_thumbnail(id) {
        Some(bytes) => {
            let encoded = general_purpose::STANDARD.encode(bytes);
            Response::Ok { data: Some(ResponseData::Thumbnail { id: id_hex, bytes: encoded }) }
        },
        None => Response::Error { message: "thumbnail not available".into() }
    }
}