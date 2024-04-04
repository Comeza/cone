# CONE 
CONE (Connection Engine) is a Framework for building TCP-based Games.

## Example

```rs

// Protocl that implements [std::fmt::Display] and [std::str::FromStr]
enum Protocol {
    Hey(String)
}

async fn main() -> std::io::Result<()> {
    let (channel, server) = Server::<Protocol>::new();
    let mut conman = Conman::new(channel);

    tokio::spawn(async move {
        if let Err(err) = conman.run("0.0.0.0:1312").await {
            println!("Conman crashed! Reason: {err}");
        }
    });
}
```
