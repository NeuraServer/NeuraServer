// ----------------------------------
// --- NeuraServer Copyright 2024 ---
// ----------------------------------

use actix_web::{web, App, HttpServer, HttpResponse, Responder, middleware};
use std::sync::{Arc, Mutex};
use std::env;
use log::info;
use dotenv::dotenv;

struct AppState {
    connection_count: Mutex<i32>,
}

async fn index(data: web::Data<Arc<AppState>>) -> impl Responder {
    let mut count = data.connection_count.lock().unwrap();
    *count += 1;
    HttpResponse::Ok().body(format!("Hello, visitor number: {}", count))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize logger
    env_logger::init();

    // Read server configuration from environment variables
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "5500".to_string());
    let server_address = format!("{}:{}", host, port);

    let workers = env::var("WORKERS").unwrap_or_else(|_| "4".to_string()).parse().unwrap_or(4); // Number of worker threads
    let max_connections = env::var("MAX_CONNECTIONS").unwrap_or_else(|_| "10000".to_string()).parse().unwrap_or(10_000); // Maximum number of concurrent connections
    let max_connection_rate = env::var("MAX_CONNECTION_RATE").unwrap_or_else(|_| "256".to_string()).parse().unwrap_or(256); // Maximum connection rate per second
    let backlog = env::var("BACKLOG").unwrap_or_else(|_| "128".to_string()).parse().unwrap_or(128); // Number of pending connections
    let max_payload_size = env::var("MAX_PAYLOAD_SIZE").unwrap_or_else(|_| "10485760".to_string()).parse().unwrap_or(10 * 1024 * 1024); // Maximum payload size in bytes

    let data = web::Data::new(Arc::new(AppState {
        connection_count: Mutex::new(0),
    }));

    info!("Starting server at {}", server_address);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .service(web::resource("/").to(index))
            .service(web::resource("/health").to(|| HttpResponse::Ok().body("Server is running")))
    })
    .workers(workers)
    .max_connections(max_connections)
    .max_connection_rate(max_connection_rate)
    .backlog(backlog)
    .max_payload_size(max_payload_size)
    .bind(server_address)?
    .run()
    .await
}


