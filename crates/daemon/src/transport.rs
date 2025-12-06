use tokio::io::{AsyncRead, AsyncWrite};

pub trait Transport: AsyncRead + AsyncWrite + Unpin + Send + 'static {}

impl<T> Transport for T
where
    T: AsyncRead + AsyncWrite + Unpin + Send + 'static
{}