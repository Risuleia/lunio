mod protocol;
mod daemon;
mod server;
mod commands;

use lunio_core::EngineRuntime;

use crate::{daemon::Daemon, server::start_server};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let engine = EngineRuntime::new(".lunio-cache".into());

    println!("[lunio-daemon] scanning...");
    engine.full_scan(".");

    let daemon = Daemon::new(engine);
    start_server(daemon).await?;

    Ok(())
}
