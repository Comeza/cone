use std::collections::HashMap;
use tokio::sync::mpsc::{self, Receiver, Sender, UnboundedSender};

mod client;
mod shaker;

pub use client::*;
pub use shaker::*;

struct Envelope<T> {
    source: u32,
    msg: T,
}

struct Server<ServerMsg, ClientMsg> {
    clients: HashMap<u32, UnboundedSender<ServerMsg>>,
    inbox: Receiver<Envelope<ClientMsg>>,
    next_client_id: u32,
}

impl<SMsg, CMsg> Server<SMsg, CMsg> {
    fn new() -> Self {
        // let (sender, receiver) = mpsc::unbounded_channel();
        Self {
            clients: HashMap::new(),
            next_client_id: 0,
            inbox: todo!(),
        }
    }

    fn bump_id(&mut self) -> u32 {
        self.next_client_id += 1;
        self.next_client_id - 1
    }

    fn insert_client(&mut self, client: UnboundedSender<SMsg>) -> u32 {
        let id = self.bump_id();
        self.clients.insert(id, client);
        id
    }
}
