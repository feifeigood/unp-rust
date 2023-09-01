use chrono::Utc;
use tokio::{io::AsyncWriteExt, net::TcpListener};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:8013").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let ticks = Utc::now().format("%Y-%m-%d %H:%M:%S");
            _ = socket.write(ticks.to_string().as_bytes()).await;
            socket.shutdown().await.expect("close connection error");
        });
    }
}
