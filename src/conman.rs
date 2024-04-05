use std::{collections::HashMap, fmt::Display, str::FromStr, sync::Arc, time::Duration};

use serde::{Deserialize, Serialize};
use tokio::{
    io::BufStream,
    net::{TcpListener, ToSocketAddrs},
    sync::{mpsc::UnboundedSender, Mutex},
};

use crate::{ext::SplitUtil, Client};

pub enum ConmanSignal<P>
where
    P: Serialize + Deserialize<'static>,
{
    JoinRequest(Client<P>),
}

pub struct Conman<P>
where
    P: Serialize + Deserialize<'static> + 'static,
{
    channel: UnboundedSender<ConmanSignal<P>>,
    users: HashMap<String, String>,
    motd: Option<String>,
    connection_timeout: Duration,
}

impl<P> Conman<P>
where
    P: Serialize + Deserialize<'static> + Send + 'static,
{
    pub fn new(channel: UnboundedSender<ConmanSignal<P>>) -> Self {
        Self {
            motd: None,
            channel,
            connection_timeout: Duration::from_secs(3),
            users: HashMap::new(),
        }
    }

    pub fn with_timeout(self, connection_timeout: Duration) -> Self {
        Self {
            connection_timeout,
            ..self
        }
    }

    pub fn with_motd<S: Into<String>>(self, motd: Option<S>) -> Self {
        Self {
            motd: motd.map(|x| x.into()),
            ..self
        }
    }

    pub fn with_channel(self, channel: UnboundedSender<ConmanSignal<P>>) -> Self {
        Self { channel, ..self }
    }

    fn check(&mut self, username: String, password: String) -> bool {
        match self.users.get(&username) {
            Some(pwd) if *pwd == password => true,
            None => {
                self.users.insert(username, password);
                true
            }
            _ => false,
        }
    }

    pub async fn run(self, addr: impl ToSocketAddrs) -> std::io::Result<()> {
        let listener = TcpListener::bind(addr).await?;
        let motd_line = self.motd.clone();
        let conman = Arc::new(Mutex::new(self));

        loop {
            let (stream, addr) = listener.accept().await?; // fix, do not use ? here
            let conman = conman.clone();
            let motd = motd_line.clone();

            let _ = tokio::spawn(async move {
                if let Err(err) = tokio::time::timeout(Duration::from_secs(3), async move {
                    println!("New connection request from {addr}");
                    let stream = BufStream::new(stream);
                    let mut client = Client::<P>::new(stream);

                    if let Some(motd) = motd {
                        client.writeln(&format!("MOTD {motd}\n")).await?;
                    }

                    let next_line = client.read_next().await?;
                    let next_line = next_line.as_str();
                    let Some(("LOGIN", username, password)) = next_line.split_twice(' ') else {
                        return std::io::Result::Ok(());
                    };

                    {
                        let mut conman = conman.lock().await;
                        if conman.check(username.to_string(), password.to_string()) {
                            client.writeln("AUTH OK").await?;
                            println!("AUTH OK for {addr}");
                            conman.channel.send(ConmanSignal::JoinRequest(client)).unwrap();
                        } else {
                            println!("AUTH NO OK :( for {addr}");
                        }
                    }

                    Ok(())
                })
                .await
                {
                    println!("Err: {err}");
                };
            });
        }
    }
}
