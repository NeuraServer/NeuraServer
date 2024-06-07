// ----------------------------------
// --- NeuraServer Copyright 2024 ---
// ----------------------------------

mod server;
mod asm;

use server::start_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    start_server().await
}
