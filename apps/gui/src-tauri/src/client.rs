use std::sync::Mutex;

use lunio_client::Client;
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

pub static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().unwrap());
static CLIENT: Lazy<Mutex<Option<Client>>> = Lazy::new(|| Mutex::new(None));

pub fn connect() -> anyhow::Result<()> {
    let client = RUNTIME.block_on(Client::connect())?;
    *CLIENT.lock().unwrap() = Some(client);

    Ok(())
}

pub fn with_client<F, R>(f: F) -> anyhow::Result<R>
where
    F: FnOnce(&mut Client) -> anyhow::Result<R>
{
    let mut guard = CLIENT.lock().unwrap();
    let client = guard.as_mut().ok_or_else(|| anyhow::anyhow!("Client not connected"))?;
    
    f(client)
}