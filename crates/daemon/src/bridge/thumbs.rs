use lunio_engine::thumbs::model::{ThumbResult, ThumbStatus};
use lunio_protocol::{Event, Topic};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::registry::Registry;

pub async fn bridge_thumbs(
    reg: Arc<Mutex<Registry>>,
    mut rx: tokio::sync::mpsc::Receiver<ThumbResult>,
) {
    while let Some(res) = rx.recv().await {
        let evt: Event = match res.status {
            ThumbStatus::Queued => Event::ThumbStarted {
                id: res.id.0,
                path: String::new(),
            },

            ThumbStatus::Running => Event::ThumbStarted {
                id: res.id.0,
                path: String::new(),
            },

            ThumbStatus::Completed(path) => {
                Event::ThumbReady {
                    id: res.id.0,
                    path: String::new(), // TODO fill later
                    thumb_path: path.to_string_lossy().to_string(),
                }
            }

            ThumbStatus::Failed(err) => Event::ThumbFailed {
                id: res.id.0,
                path: String::new(),
                reason: err,
            },

            ThumbStatus::Cancelled => {
                // No protocol event exists -> ignore silently
                continue;
            }
        };

        let reg = reg.lock().await;
        reg.broadcast(Topic::Thumbnails, evt).await;
    }
}
