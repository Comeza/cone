

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let mut connections = Vec::new();

    for i in 0..1000 {
        if let Err(err) = async {
            let mut stream = TcpStream::connect("127.0.0.1:1312").await?;
            stream.write_all(b"LOGIN A A\n").await?;
            stream.flush().await?;
            connections.push(stream);
            std::io::Result::Ok(())
        }
        .await
        {
            println!("Error: (Open: {i}) {err}");
        } else {
            println!("Spawned connection {i}");
        }
    }
}
