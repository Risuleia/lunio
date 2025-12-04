mod protocol;
mod daemon;
mod server;
mod commands;
mod bootstrap;

use lunio_core::EngineRuntime;

use crate::{bootstrap::{bootstrap, load_manifest}, daemon::Daemon, server::start_server};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let runtime_root = dirs::data_dir()
        .ok_or_else(|| anyhow::anyhow!("failed to locate user data directory"))?
        .join("Lunio/runtime");
    let manifest = load_manifest().await?;

    let runtime = bootstrap(runtime_root, manifest).await?;

    let engine = EngineRuntime::new(".lunio-cache".into(), runtime.ffmpeg, runtime.pdfium);

    println!("[lunio-daemon] scanning...");
    engine.full_scan(".");

    let daemon = Daemon::new(engine);
    start_server(daemon).await?;

    Ok(())
}
