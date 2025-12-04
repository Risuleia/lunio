use anyhow::{Result, anyhow};
use futures::stream::{AbortHandle, AbortRegistration, Abortable};
use lunio_client::{Client, FileEntry};
use once_cell::sync::Lazy;
use tokio::sync::{Mutex, mpsc, oneshot};

static CLIENT_TX: Lazy<Mutex<Option<mpsc::Sender<ClientCmd>>>> = Lazy::new(|| Mutex::new(None));

pub enum ClientCmd {
    Search {
        query: String,
        limit: Option<usize>,
        respond: oneshot::Sender<Result<Vec<FileEntry>>>,
    },
    ListDir {
        path: String,
        abort: AbortHandle,
        abort_reg: AbortRegistration,
        respond: oneshot::Sender<Result<Vec<FileEntry>>>,
    },
    RequestThumbnail {
        id: String,
        respond: oneshot::Sender<Result<()>>,
    },
    GetThumbnail {
        id: String,
        respond: oneshot::Sender<Result<Vec<u8>>>,
    },
    OpenFile {
        path: String,
        respond: oneshot::Sender<Result<()>>,
    },
    Shutdown {
        respond: oneshot::Sender<Result<()>>,
    }
}

pub async fn connect() -> Result<()> {
    let (tx, mut rx) = mpsc::channel::<ClientCmd>(64);
    let mut client = Client::connect().await?;

    tokio::spawn(async move {
        let mut current_list_abort: Option<AbortHandle> = None;

        while let Some(cmd) = rx.recv().await {
            match cmd {
                ClientCmd::Search { query, limit, respond } => {
                    let r = client.search(query, limit).await;
                    let _ = respond.send(r);
                }

                ClientCmd::ListDir { path, respond, abort, abort_reg } => {
                    if let Some(old) = current_list_abort.take() {
                        old.abort();
                    }

                    current_list_abort = Some(abort);

                    let future = client.list_dir(path);
                    let abortable = Abortable::new(future, abort_reg);

                    let r = match abortable.await {
                        Ok(res) => res,
                        Err(_) => Ok(vec![])
                    };

                    let _ = respond.send(r);
                    current_list_abort = None;
                }

                ClientCmd::RequestThumbnail { id, respond } => {
                    let r = client.request_thumbnail(id).await;
                    let _ = respond.send(r);
                }

                ClientCmd::GetThumbnail { id, respond } => {
                    let r = client.get_thumbnail(id).await;
                    let _ = respond.send(r);
                }

                ClientCmd::OpenFile { path, respond } => {
                    let r = client.open_file(path).await;
                    let _ = respond.send(r);
                }

                ClientCmd::Shutdown { respond } => {
                    let r = client.shutdown().await;
                    let _ = respond.send(r);
                    break;
                }
            }
        }
    });

    *CLIENT_TX.lock().await = Some(tx);
    Ok(())
}

async fn send<R>(
    f: impl FnOnce(oneshot::Sender<Result<R>>) -> ClientCmd
) -> Result<R> {
    let tx = CLIENT_TX
        .lock().await
        .as_ref()
        .ok_or_else(|| anyhow!("Client not connected"))?
        .clone();

    let (res_tx, res_rx) = oneshot::channel();

    tx.send(f(res_tx))
        .await
        .map_err(|_| anyhow!("Client worker is offline"))?;

    res_rx.await?
}

async fn send_abortable<R>(
    f: impl FnOnce(AbortHandle, AbortRegistration, oneshot::Sender<Result<R>>) -> ClientCmd
) -> Result<R> {
    let tx = CLIENT_TX
        .lock().await
        .as_ref()
        .ok_or_else(|| anyhow!("Client not connected"))?
        .clone();

    let (abort, abort_reg) = AbortHandle::new_pair();
    let (res_tx, res_rx) = oneshot::channel();

    tx.send(f(abort, abort_reg, res_tx))
        .await
        .map_err(|_| anyhow!("Client worker is offline"))?;

    res_rx.await?
}

pub async fn search(query: String, limit: Option<usize>) -> Result<Vec<FileEntry>> {
    send(|respond| ClientCmd::Search { query, limit, respond }).await
}

pub async fn list_dir(path: String) -> Result<Vec<FileEntry>> {
    send_abortable(|abort, abort_reg, respond| ClientCmd::ListDir { path, abort, respond, abort_reg }).await
}

pub async fn request_thumbnail(id: String) -> Result<()> {
    send(|respond| ClientCmd::RequestThumbnail { id, respond }).await
}

pub async fn get_thumbnail(id: String) -> Result<Vec<u8>> {
    send(|respond| ClientCmd::GetThumbnail { id, respond }).await
}

pub async fn open_file(path: String) -> Result<()> {
    send(|respond| ClientCmd::OpenFile { path, respond }).await
}

pub async fn shutdown() -> Result<()> {
    send(|respond| ClientCmd::Shutdown { respond }).await
}