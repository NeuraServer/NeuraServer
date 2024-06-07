// ----------------------------------
// --- NeuraServer Copyright 2024 ---
// ----------------------------------

use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::asm::asm_example;

pub async fn handle_client(mut socket: TcpStream) {
    let mut buf = [0; 1024];

    loop {
        let n = match socket.read(&mut buf).await {
            Ok(n) if n == 0 => return,
            Ok(n) => n,
            Err(_) => return,
        };

        asm_example();

        if socket.write_all(&buf[0..n]).await.is_err() {
            return;
        }
    }
}

