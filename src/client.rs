use std::{future::Future, net::SocketAddr};

use tokio::{
    io::BufStream,
    net::TcpStream,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
};

use crate::Envelope;

pub trait Client<ServerMsg, ClientMsg> {
    fn send_to_host(&self, msg: ClientMsg);
    fn get_sender(&self) -> UnboundedSender<ServerMsg>;
    fn run(self) -> impl Future<Output = ()> + Send;
}

struct DefaultClient<ServerMsg, ClientMsg> {
    id: u32,
    addr: SocketAddr,
    stream: BufStream<TcpStream>,
    /// Messages from the Server
    receiver: UnboundedReceiver<ServerMsg>,

    host_sender: UnboundedSender<ServerMsg>,
    /// Messages from the client
    sender: UnboundedSender<Envelope<ClientMsg>>,
}

impl<ServerMsg, ClientMsg> Client<ServerMsg, ClientMsg> for DefaultClient<ServerMsg, ClientMsg> {
    fn send_to_host(&self, msg: ClientMsg) {
        let _ = self.sender.send(Envelope {
            source: self.id,
            msg,
        });
    }

    fn run(self) -> impl Future<Output = ()> {
        async {}
    }

    fn get_sender(&self) -> UnboundedSender<ServerMsg> {
        self.host_sender.clone()
    }
}
