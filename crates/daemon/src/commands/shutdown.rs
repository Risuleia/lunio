use crate::protocol::Response;

pub async fn handle_shutdown() -> Response {
    Response::Ok { results: None }
}