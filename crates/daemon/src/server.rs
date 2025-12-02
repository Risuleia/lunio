use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};

use crate::{daemon::Daemon, protocol::{Handshake, PROTOCOL_VERSION, Request, Response}};

const MAX_PACKET: usize = 8 * 1024 * 1024;

async fn handle_connection(daemon: Daemon, mut socket: TcpStream) {
    let hello = Handshake {
        protocol: PROTOCOL_VERSION,
        engine: "lunio-daemon".into()
    };

    let hello_bytes = serde_json::to_vec(&hello).unwrap();
    let _ = socket.write_u32(hello_bytes.len() as u32).await;
    let _ = socket.write_all(&hello_bytes).await;

    loop {
        let len = match socket.read_u32().await {
            Ok(l) => l as usize,
            Err(_) => return
        };

        if len > MAX_PACKET {
            eprintln!("Packet too large: {}", len);
            return;
        }

        let mut buf = vec![0u8; len];

        if socket.read_exact(&mut buf).await.is_err() {
            return;
        }

        let request: Request = match serde_json::from_slice(&buf) {
            Ok(r) => r,
            Err(err) => {
                let resp = Response::Error { message: err.to_string() };
                let out = serde_json::to_vec(&resp).unwrap();
                let _ = socket.write_u32(out.len() as u32).await;
                let _ = socket.write_all(&out).await;

                continue;
            }
        };

        let response = daemon.dispatch(request).await;
        let out = serde_json::to_vec(&response).unwrap();

        let _ = socket.write_u32(out.len() as u32).await;
        let _ = socket.write_all(&out).await;
    }

}

pub async fn start_server(daemon: Daemon) -> anyhow::Result<()> {
    println!("[lunio-daemon] running on localhost:9000");
    let listener = TcpListener::bind("localhost:9000").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        let daemon_clone = daemon.clone();

        tokio::spawn(async move {
            handle_connection(daemon_clone, socket).await;
        });
    }
}