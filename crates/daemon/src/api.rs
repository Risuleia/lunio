use lunio_engine::jobs::scheduler::SchedulerCommand;
use tokio::sync::{Mutex, mpsc};
use std::sync::Arc;
use tokio::io::split;
use uuid::Uuid;

use lunio_protocol::{Envelope, Command, Event};
use crate::framing::{read_frame, write_frame};
use crate::registry::Registry;
use crate::router::handle_command;
use crate::transport::Transport;

pub async fn handle_client<S>(
    stream: S,
    registry: Arc<Mutex<Registry>>,
    job_tx: mpsc::Sender<SchedulerCommand>,
)
where
    S: Transport,
{
    let session_id = Uuid::new_v4();

    let (mut reader, mut writer) = split(stream);

    let (tx, mut rx) = mpsc::channel::<Envelope<Event>>(256);

    // ✅ Register client
    registry.lock().await.register(session_id, tx.clone());

    // ✅ Writer task
    let write_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if write_frame(&mut writer, &msg).await.is_err() {
                break;
            }
        }
    });

    // ✅ Reader loop
    while let Ok(incoming) = read_frame::<Command>(&mut reader).await {

        handle_command(
            session_id,
            incoming,
            registry.clone(),
            job_tx.clone(),
        ).await;
    }

    // ✅ Cleanup on disconnect
    registry.lock().await.remove(&session_id);

    write_task.abort();
}
