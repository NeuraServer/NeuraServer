// ----------------------------------
// --- NeuraServer Copyright 2024 ---
// ----------------------------------

pub mod handler;

use tokio::net::TcpListener;
use handler::handle_client;

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on 127.0.0.1:8080");

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            handle_client(socket).await;
        });
    }
}
