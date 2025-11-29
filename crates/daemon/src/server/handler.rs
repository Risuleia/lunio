use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

use crate::{daemon::Daemon, protocol::{Request, Response}};

pub async fn handle_connection(daemon: Daemon, mut socket: TcpStream) {
    let mut buf = Vec::new();

    if socket.read_to_end(&mut buf).await.is_err() {
        return;
    }

    let request: Request = match serde_json::from_slice(&buf) {
        Ok(r) => r,
        Err(err) => {
            let resp = Response::Error { message: err.to_string() };
            let _ = socket.write_all(&serde_json::to_vec(&resp).unwrap()).await;
            return;
        }
    };

    let response = daemon.dispatch(request).await;
    let _ = socket.write_all(&serde_json::to_vec(&response).unwrap()).await;
}