use std::io::Result as IoResult;
use lunio_protocol::{decode, encode};
use serde::{Serialize, de::DeserializeOwned};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub async fn write_frame<T: Serialize>(
    stream: &mut (impl AsyncWrite + Unpin),
    value: &T
) -> IoResult<()> {
    let bytes = encode(value);
    let len = bytes.len() as u32;

    stream.write_u32(len).await?;
    stream.write_all(&bytes).await?;
    stream.flush().await?;

    Ok(())
}

pub async fn read_frame<T: DeserializeOwned>(
    stream: &mut (impl AsyncRead + Unpin)
) -> IoResult<T> {
    let len = stream.read_u32().await?;
    let mut buf = vec![0u8; len as usize];

    stream.read_exact(&mut buf).await?;

    Ok(decode(&buf))
}