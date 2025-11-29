use lunio_core::EngineRuntime;

use crate::{commands::{scan::handle_scan, search::handle_search, shutdown::handle_shutdown}, protocol::{Request, Response}};

#[derive(Clone)]
pub struct Daemon {
    pub engine: EngineRuntime
}

impl Daemon {
    pub fn new(engine: EngineRuntime) -> Self {
        Self { engine }
    }

    pub async fn dispatch(&self, req: Request) -> Response {
        match req {
            Request::Search { query, limit } => handle_search(self.engine.clone(), query, limit).await,
            Request::Scan { root } => handle_scan(self.engine.clone(), root).await,
            Request::Shutdown => handle_shutdown().await
        }
    }
}