use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display, str::FromStr};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

mod client;
mod conman;
mod ext;
mod protocol;

pub use serde;
pub use tokio;

pub use client::*;
pub use conman::*;

pub struct Server<P>
where
    P: Serialize + Deserialize<'static>,
{
    clients: HashMap<u32, Client<P>>,
    receiver: UnboundedReceiver<ConmanSignal<P>>,
    next_client_id: u32,
}

impl<P> Server<P>
where
    P: Serialize + Deserialize<'static>,
{
    pub fn new() -> (UnboundedSender<ConmanSignal<P>>, Self) {
        let (sender, receiver) = mpsc::unbounded_channel();
        let server = Self {
            next_client_id: 0,
            receiver,
            clients: HashMap::new(),
        };

        (sender, server)
    }

    pub fn insert_client(&mut self, client: impl Into<Client<P>>) -> u32 {
        self.clients.insert(self.next_client_id, client.into());
        self.next_client_id += 1;
        self.next_client_id - 1
    }
}
