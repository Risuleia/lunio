use tokio::net::TcpListener;

use crate::{daemon::Daemon, server::handler::handle_connection};

mod handler;

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