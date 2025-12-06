use std::{io, path::PathBuf, sync::Arc};

use lunio_engine::jobs::scheduler::SchedulerCommand;
use tokio::sync::{Mutex, mpsc::Sender};

#[cfg(unix)]
use tokio::net::UnixListener;

#[cfg(windows)]
use tokio::net::windows::named_pipe::ServerOptions;

use crate::{api::handle_client, registry::Registry, transport::Transport};

pub async fn start_ipc(
    registry: Arc<Mutex<Registry>>,
    job_tx: Sender<SchedulerCommand>
) -> io::Result<()> {
    #[cfg(unix)]
    {
        let socket_path = socket_path();
        let _ = std::fs::remove_file(&socket_path); // remove stale socket

        let listener = UnixListener::bind(&socket_path)?;
        println!("IPC listening at {:?}", socket_path);

        loop {
            let (stream, _) = listener.accept().await?;
            spawn_client(stream, registry.clone(), job_tx.clone());
        }
    }

    
    #[cfg(windows)]
    {
        let pipe_name = r"\\.\pipe\lunio-daemon";
        println!("IPC listening at {}", pipe_name);

        loop {
            let server = ServerOptions::new().create(pipe_name)?;
            server.connect().await?;
            spawn_client(server, registry.clone(), job_tx.clone());
        }
    }
}

fn spawn_client<S>(
    stream: S,
    registry: Arc<Mutex<Registry>>,
    job_tx: Sender<SchedulerCommand>,
)
where
    S: Transport + Send + 'static,
{
    tokio::spawn(async move {
        handle_client(stream, registry, job_tx).await;
    });
}

#[cfg(unix)]
fn socket_path() -> PathBuf {
    dirs::data_dir()
        .unwrap()
        .join("Lunio")
        .join("daemon.sock")
}