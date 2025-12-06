use lunio_engine::jobs::event::JobEvent;
use lunio_protocol::{Event, Topic};
use crate::registry::Registry;
use tokio::sync::Mutex;
use std::sync::Arc;

pub async fn brige_jobs(
    registry: Arc<Mutex<Registry>>,
    mut rx: tokio::sync::mpsc::Receiver<JobEvent>,
) {
    while let Some(evt) = rx.recv().await {
        if let Some(mapped) = map(evt) {
            let reg = registry.lock().await;
            reg.broadcast(Topic::Jobs, mapped).await;
        }
    }
}

fn map(evt: JobEvent) -> Option<Event> {
    use JobEvent::*;
    match evt {
        Started { id, .. } => Some(Event::JobUpdate {
            id: id.0,
            status: "running".into(),
            done: 0,
            total: 0,
        }),

        Completed { id, .. } => Some(Event::JobUpdate {
            id: id.0,
            status: "completed".into(),
            done: 1,
            total: 1,
        }),

        Failed { id, reason, .. } => Some(Event::JobFailed {
            id: id.0,
            reason,
        }),

        Progress { id, done, total, .. } => Some(Event::JobProgress {
            id: id.0,
            done,
            total,
        }),

        _ => None
    }
}
