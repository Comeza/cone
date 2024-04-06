
use std::marker::PhantomData;


use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::{io::BufStream, net::TcpStream};

pub struct Client< P>
where
    P: Serialize + Deserialize<'static>,
{
    stream: BufStream<TcpStream>,
    _pd: PhantomData<P>,
}

impl<P> Client<P>
where
    P: Serialize + Deserialize<'static>,
{
    pub fn new(stream: BufStream<TcpStream>) -> Self {
        Self {
            stream,
            _pd: PhantomData,
        }
    }

    pub async fn read_next(&mut self) -> std::io::Result<String> {
        let mut buf = String::new();
        let _ = self.stream.read_line(&mut buf).await?;
        Ok(buf.trim_end().into())
    }

    pub async fn write(&mut self, input: &str) -> std::io::Result<()> {
        self.stream.write_all(input.as_bytes()).await?;
        self.stream.flush().await
    }

    pub async fn writeln(&mut self, input: &str) -> std::io::Result<()> {
        self.stream.write_all(input.as_bytes()).await?;
        self.stream.write_u8(b'\n').await?;
        self.stream.flush().await
    }
}
