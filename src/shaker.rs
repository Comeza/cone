use std::future::Future;
use std::io::Result;
use std::{collections::HashMap, net::SocketAddr};

use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

pub trait Shaker {
    fn new(socket_addr: impl Into<SocketAddr>) -> Self;
    fn get_motd(&self) -> String;
    fn listen(&mut self) -> impl Future<Output = Result<()>>;
}

pub struct DefaultShaker {
    motd: String,
    password_map: HashMap<String, String>,
    socket_addr: SocketAddr,
}

impl Shaker for DefaultShaker {
    fn get_motd(&self) -> String {
        self.motd.clone()
    }

    fn listen(&mut self) -> impl Future<Output = Result<()>> {
        async {
            let listener = TcpListener::bind(self.socket_addr).await?;

            loop {
                let (mut socket, _) = listener.accept().await?;
                socket.write(b"Hi!").await?;
                socket.flush().await?;
            }
        }
    }

    fn new(socket_addr: impl Into<SocketAddr>) -> Self {
        Self {
            motd: "Ja Moin".into(),
            password_map: HashMap::new(),
            socket_addr: socket_addr.into(),
        }
    }
}
