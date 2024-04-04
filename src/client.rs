use std::fmt::Display;
use std::marker::PhantomData;
use std::str::FromStr;

use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite};
use tokio::{io::BufStream, net::TcpStream};

pub struct Client<P>
where
    P: FromStr + Display,
{
    stream: BufStream<TcpStream>,
    _pd: PhantomData<P>,
}

impl<P> Client<P>
where
    P: FromStr + Display,
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
}
