use std::sync::Arc;

use lunio_core::EngineRuntime;

use crate::{commands::{get_thumbnail::handle_get_thumbnail, list_dir::handle_list_dir, open_file::handle_open_file, request_thumbnail::handle_request_thumbnail, scan::handle_scan, search::handle_search, shutdown::handle_shutdown}, protocol::{Request, Response}};

#[derive(Clone)]
pub struct Daemon {
    pub engine: Arc<EngineRuntime>
}

impl Daemon {
    pub fn new(engine: EngineRuntime) -> Self {
        Self { engine: Arc::new(engine) }
    }

    pub async fn dispatch(&self, req: Request) -> Response {
        match req {
            Request::Search { query, limit } => handle_search(self.engine.clone(), query, limit).await,
            Request::Scan { root } => handle_scan(self.engine.clone(), root).await,
            Request::ListDir { path } => handle_list_dir(self.engine.clone(), path).await,
            Request::RequestThumbnail { id } => handle_request_thumbnail(self.engine.clone(), id).await,
            Request::GetThumbnail { id } => handle_get_thumbnail(self.engine.clone(), id).await,
            Request::OpenFile { path } => handle_open_file(self.engine.clone(), path).await,
            Request::Shutdown => handle_shutdown(self.engine.clone()).await
        }
    }
}