use serde::{Deserialize, Serialize};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

#[derive(Serialize)]
#[serde{tag = "type"}]
pub enum Request<'a> {
    Search { query: &'a str, limit: Option<usize> }
}

#[derive(Deserialize)]
#[serde(tag = "status")]
pub enum Response {
    #[serde(rename = "ok")]
    Ok { results: Vec<FileEntry> },

    #[serde(rename = "error")]
    Err { message: String },
}


#[derive(Serialize, Deserialize, Debug)]
pub struct FileEntry {
    pub path: String,
    pub size: u64,
}

pub async fn search(query: &str, limit: Option<usize>) -> anyhow::Result<Vec<FileEntry>> {
    let mut stream = TcpStream::connect("localhost:9000").await?;

    let req = Request::Search { query, limit };
    stream.write_all(&serde_json::to_vec(&req)?).await?;
    stream.shutdown().await?;

    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await?;

    match serde_json::from_slice::<Response>(&buf)? {
        Response::Ok { results } => Ok(results),
        Response::Err { message } => Err(anyhow::anyhow!(message))
    }
}