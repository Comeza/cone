use cone::{DefaultShaker, Server, Shaker};

enum ServerMessage {
    Motd(),
}

enum ClientMessage {
    Login(String, String),
}

#[tokio::main]
async fn main() {
    let server = Server::<ServerMessage, ClientMessage>::new();
}
