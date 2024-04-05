use std::{collections::HashMap, fmt::Display, str::FromStr, sync::Arc, time::Duration};

use tokio::{
    io::BufStream,
    net::{TcpListener, ToSocketAddrs},
    sync::{mpsc::UnboundedSender, Mutex},
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
    P: FromStr + Display + Send + 'static,
{
    pub fn new(channel: UnboundedSender<ConmanSignal<P>>) -> Self {
        Self {
            channel,
            users: HashMap::new(),
        }
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
        let conman = Arc::new(Mutex::new(self));

        loop {
            let (stream, addr) = listener.accept().await?; // fix, do not use ? here
            let conman = conman.clone();

            let _ = tokio::spawn(async move {
                if let Err(err) = tokio::time::timeout(Duration::from_secs(3), async move {
                    println!("New connection request from {addr}");
                    let stream = BufStream::new(stream);
                    let mut client = Client::<P>::new(stream);

                    // This is not good! One client can just write nothing and starve all other requests
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
                }).await {

                    println!("Err: {err}");
    };
            });
        }
    }
}
