use std::{fmt::Display, str::FromStr, time::Duration};

use cone::{Conman, Server};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum Protocol {
    Hey(String),
}

enum ProtocolError {
    WrongFormat,
    UnknownCommand,
}

impl FromStr for Protocol {
    type Err = ProtocolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {
            Some(("HEY", msg)) => Ok(Self::Hey(msg.into())),
            _ => Err(ProtocolError::UnknownCommand),
        }
    }
}

impl Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::Hey(msg) => write!(f, "HEY {msg}"),
        }
    }
}

#[tokio::main]
async fn main() {
    let (channel, server) = Server::<Protocol>::new();

    let conman = Conman::new(channel)
        .with_motd(Some("Welcome to the example server!"))
        .with_timeout(Duration::from_secs(10));

    tokio::spawn(async move {
        if let Err(err) = conman.run("0.0.0.0:1312").await {
            println!("Conman crashed! Reason: {err}");
        }
    });

    loop {}
}
