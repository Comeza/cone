use cone::{DefaultShaker, Shaker};

#[tokio::main]
async fn main() {
    let shaker = DefaultShaker::new("0.0.0.0:1312");
}
