use anyhow::{Result, anyhow};
use base64::{Engine, engine::general_purpose};
use serde::{Deserialize, Serialize};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

const ADDR: &str = "localhost:9000";

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Request {
    Scan { root: String },
    Search { query: String, limit: Option<usize> },
    ListDir { path: String },
    RequestThumbnail { id: String },
    GetThumbnail { id: String },
    OpenFile { path: String },
    Shutdown
}

#[derive(Deserialize)]
#[serde(tag = "status")]
pub enum Response {
    #[serde(rename = "ok")]
    Ok { data: Option<ResponseData> },

    #[serde(rename = "error")]
    Error { message: String },
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ResponseData {
    SearchResults { entries: Vec<FileEntry> },
    DirectoryListing { entries: Vec<FileEntry> },
    Thumbnail { id: String, bytes: String },
    Ack,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileEntry {
    pub id: String,
    pub path: String,
    pub size: u64,
    pub is_dir: bool,
    pub modified: Option<i64>,
    pub has_thumbnail: bool,
}


#[derive(Deserialize, Debug)]
pub struct Handshake {
    pub protocol: u8,
    pub engine: String,
}

pub struct Client {
    socket: TcpStream
}

impl Client {
    pub async fn connect() -> Result<Self> {
        let mut socket = TcpStream::connect(ADDR).await?;

        let len = socket.read_u32().await?;
        let mut buf = vec![0u8; len as usize];
        socket.read_exact(&mut buf).await?;
        let hello: Handshake = serde_json::from_slice(&buf)?;

        println!("[lunio] connected to {} (protocol {})", hello.engine, hello.protocol);

        Ok(Self { socket })
    }

    async fn send(&mut self, req: Request) -> Result<Response> {
        let payload = serde_json::to_vec(&req)?;
        self.socket.write_u32(payload.len() as u32).await?;
        self.socket.write_all(&payload).await?;

        let len = self.socket.read_u32().await?;
        let mut buf = vec![0u8; len as usize];
        self.socket.read_exact(&mut buf).await?;

        let resp: Response = serde_json::from_slice(&buf)?;
        Ok(resp)
    }
    
    pub async fn search(&mut self, query: impl Into<String>, limit: Option<usize>) -> Result<Vec<FileEntry>> {
        let resp = self.send(Request::Search {
            query: query.into(),
            limit,
        }).await?;

        match resp {
            Response::Ok { data: Some(ResponseData::SearchResults { entries }) } => Ok(entries),
            Response::Ok { .. } => Err(anyhow!("unexpected response")),
            Response::Error { message } => Err(anyhow!(message)),
        }
    }

    pub async fn scan(&mut self, root: impl Into<String>) -> Result<()> {
        let resp = self.send(Request::Scan { root: root.into() }).await?;

        match resp {
            Response::Ok { data: Some(ResponseData::Ack) } => Ok(()),
            Response::Error { message } => Err(anyhow!(message)),
            _ => Err(anyhow!("invalid response")),
        }
    }
    
    pub async fn list_dir(&mut self, path: impl Into<String>) -> Result<Vec<FileEntry>> {
        let resp = self.send(Request::ListDir { path: path.into() }).await?;

        match resp {
            Response::Ok { data: Some(ResponseData::DirectoryListing { entries }) } => Ok(entries),
            Response::Error { message } => Err(anyhow!(message)),
            _ => Err(anyhow!("invalid response")),
        }
    }

    pub async fn request_thumbnail(&mut self, id: String) -> Result<()> {
        let resp = self.send(Request::RequestThumbnail { id }).await?;

        match resp {
            Response::Ok { data: Some(ResponseData::Ack) } => Ok(()),
            Response::Error { message } => Err(anyhow!(message)),
            _ => Err(anyhow!("invalid response")),
        }
    }

    pub async fn get_thumbnail(&mut self, id: String) -> Result<Vec<u8>> {
        let resp = self.send(Request::GetThumbnail { id: id.clone() }).await?;

        match resp {
            Response::Ok { data: Some(ResponseData::Thumbnail { bytes, .. }) } => {
                let decoded = general_purpose::STANDARD.decode(bytes)?;
                Ok(decoded)
            }
            Response::Error { message } => Err(anyhow!(message)),
            _ => Err(anyhow!("invalid response")),
        }
    }

    pub async fn open_file(&mut self, path: String) -> Result<()> {
        let resp = self.send(Request::OpenFile { path: path.clone() }).await?;

        match resp {
            Response::Ok { data: Some(ResponseData::Ack) } => Ok(()),
            Response::Error { message } => Err(anyhow!(message)),
            _ => Err(anyhow!("invalid response"))
        }
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        let resp = self.send(Request::Shutdown).await?;

        match resp {
            Response::Ok { data: Some(ResponseData::Ack) } => Ok(()),
            _ => Err(anyhow!("shutdown failed")),
        }
    }
}