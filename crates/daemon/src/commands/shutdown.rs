use std::{sync::Arc, time::Duration};

use lunio_core::EngineRuntime;

use crate::protocol::{Response, ResponseData};

pub async fn handle_shutdown(engine: Arc<EngineRuntime>) -> Response {
    engine.shutdown();
    tokio::spawn(async {
        tokio::time::sleep(Duration::from_millis(100)).await;
        std::process::exit(0);
    });
    
    Response::Ok { data: Some(ResponseData::Ack) }
}