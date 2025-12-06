use std::{sync::Arc};

use lunio_engine::{jobs, thumbs::{extractor::ExtractorEnv, service::ThumbService}, watch::WatchEvent};
use lunio_protocol::{ChangeKind, Envelope, Event, Topic};
use tokio::sync::{Mutex, mpsc};

use crate::{bootstrap::{bootstrap, load_manifest}, bridge::{bridge_thumbs, brige_jobs}, ipc::start_ipc, registry::Registry};

mod bootstrap;
mod api;
mod dispatcher;
mod registry;
mod router;
mod transport;
mod framing;
mod event;
mod bridge;
mod ipc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let runtime_root = dirs::data_dir()
        .ok_or_else(|| anyhow::anyhow!("failed to locate user data directory"))?
        .join("Lunio/runtime");
    let data_dir = dirs::data_dir()
        .ok_or_else(|| anyhow::anyhow!("failed to locate user data directory"))?
        .join("Lunio/data");

    let manifest = load_manifest().await?;
    let runtime = bootstrap(runtime_root.clone(), manifest).await?;

    let extractor_env = ExtractorEnv {
        ffmpeg: runtime.ffmpeg.clone(),
        pdfium: runtime.pdfium.clone()
    };

    let (event_tx, event_rx) = mpsc::channel(512);

    let job_tx = jobs::scheduler::Scheduler::start(
        4,
        &data_dir,
        event_tx.clone()
    ).await;

    let registry = Arc::new(Mutex::new(Registry::new()));
    
    tokio::spawn({
        let reg = registry.clone();
        async move {
            brige_jobs(reg, event_rx).await;
        }
    });

    
    let (thumbs, thumb_rx) = ThumbService::start(
        4,
        runtime_root.join("thumbs"),
        extractor_env,
    );

    tokio::spawn({
        let reg = registry.clone();
        async move {
            bridge_thumbs(reg, thumb_rx).await;
        }
    });

    tokio::spawn({
        let reg: Arc<Mutex<Registry>> = registry.clone();
        let jobs = job_tx.clone();
        async move {
            if let Err(e) = start_ipc(reg, jobs).await {
                eprintln!("IPC failed: {:?}", e);
            }
        }
    });

    Ok(())
}


// fn map_to_protocol(evt: WatchEvent) -> crate::dispatcher::DaemonEvent {
//     let (path, kind) = match evt {
//         WatchEvent::Created(p) => (p, ChangeKind::Created),
//         WatchEvent::Modified(p) => (p, ChangeKind::Modified),
//         WatchEvent::Deleted(p) => (p, ChangeKind::Deleted),
//     };

//     crate::dispatcher::DaemonEvent {
//         topic: Topic::FileSystem,
//         event: Envelope::new(/* session */ todo!(), Event::FileChanged {
//             path: path.to_string_lossy().to_string(),
//             kind,
//         }),
//     }
// }