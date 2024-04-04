use std::collections::HashMap;
use tokio::{
    sync::mpsc::{Receiver, UnboundedSender},
    task::JoinHandle,
};

mod client;
mod shaker;

pub use client::*;
pub use shaker::*;

struct Envelope<T> {
    source: u32,
    msg: T,
}

pub struct Server<ServerMsg, ClientMsg> {
    clients: HashMap<u32, UnboundedSender<ServerMsg>>,
    client_threads: HashMap<u32, JoinHandle<()>>,
    inbox: Receiver<Envelope<ClientMsg>>,
    next_client_id: u32,
}

impl<ServerMsg, ClientMsg> Server<ServerMsg, ClientMsg> {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
            client_threads: HashMap::new(),
            next_client_id: 0,
            inbox: todo!(),
        }
    }

    fn bump_id(&mut self) -> u32 {
        self.next_client_id += 1;
        self.next_client_id - 1
    }

    pub fn insert_client(
        &mut self,
        client: impl Client<ServerMsg, ClientMsg> + Send + 'static,
    ) -> u32 {
        let id = self.bump_id();
        self.clients.insert(id, client.get_sender());

        let thread = tokio::spawn(async { client.run().await });
        self.client_threads.insert(id, thread);

        id
    }
}
