use std::{collections::HashMap, fmt::Display, str::FromStr};

use tokio::{
    io::BufStream,
    net::{TcpListener, ToSocketAddrs},
    sync::mpsc::UnboundedSender,
};

use crate::{ext::SplitUtil, Client};

pub enum ConmanSignal<P>
where
    P: FromStr + Display,
{
    JoinRequest(Client<P>),
}

pub struct Conman<P>
where
    P: FromStr + Display,
{
    channel: UnboundedSender<ConmanSignal<P>>,
    users: HashMap<String, String>,
}

impl<P> Conman<P>
where
    P: FromStr + Display,
{
    pub fn new(channel: UnboundedSender<ConmanSignal<P>>) -> Self {
        Self {
            channel,
            users: HashMap::new(),
        }
    }

    pub async fn run(&mut self, addr: impl ToSocketAddrs) -> std::io::Result<()> {
        let listener = TcpListener::bind(addr).await?;

        loop {
            let (stream, addr) = listener.accept().await?; // fix, do not use ? here
            println!("New connection request from {addr}");
            let stream = BufStream::new(stream);
            let mut client = Client::<P>::new(stream);

            // This is not good! One client can just write nothing and starve all other requests
            let next_line = client.read_next().await?;
            let next_line = next_line.as_str();
            let Some(("LOGIN", username, password)) = next_line.split_twice(' ') else {
                continue;
            };

            let pwd = self.users.entry(username.to_owned()).or_insert(password.to_string());
            if pwd == password {
                println!("Auth passed for {addr}. Sending JoinReq");
                self.channel.send(ConmanSignal::JoinRequest(client)).unwrap();
                continue;
            }

            println!("Auth failed for {addr}.");
        }
    }
}
