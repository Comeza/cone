use std::future::Future;
use std::io::{Error, Result};
use std::collections::HashMap;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufStream};
use tokio::net::{TcpListener, ToSocketAddrs};

const MOTD: &str = "Ja Moin";

pub trait Shaker {
    fn new() -> Self;
    fn listen(&mut self, socket_addr: impl ToSocketAddrs) -> impl Future<Output = Result<()>>;
}

pub struct DefaultShaker {
    password_map: HashMap<String, String>,
}

impl Shaker for DefaultShaker {
    fn listen(&mut self, socket_addr:impl ToSocketAddrs) -> impl Future<Output = Result<()>> {
        async {
            let motd_packet = format!("MOTD {}\n", MOTD);
            let listener = TcpListener::bind(socket_addr).await?;

            loop {
                let (mut socket, _) = listener.accept().await?;
                let mut stream = BufStream::new(&mut socket);
                stream.write(motd_packet.clone().as_bytes()).await?;
                stream.flush().await?;

                let mut line = String::new();
                stream.read_line(&mut line).await?;
                println!("Received: {}", line);
            }
        }
    }

    fn new() -> Self {
        Self {
            password_map: HashMap::new(),
        }
    }
}
