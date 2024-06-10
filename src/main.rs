use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::sync::{Arc, Mutex};
use std::env;
use dotenv::dotenv;
use log::info;
use env_logger;

// Struct to store application state
struct AppState {
    connection_count: Mutex<i64>,
}

// Handler for the index route
async fn index(data: web::Data<Arc<AppState>>) -> impl Responder {
    let mut count = data.connection_count.lock().unwrap();
    *count += 1;
    HttpResponse::Ok().body(format!("Hello, visitor number: {}", count))
}

// Handler for the health check route
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is running")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize logger
    env_logger::init();

    // Read server configuration from environment variables
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "5500".to_string());

    // Configuration options
    let server_address = format!("{}:{}", host, port);
    let workers = 4; // Number of worker threads
    let max_connections = 10_000; // Maximum number of concurrent connections
    let max_connection_rate = 256; // Maximum connection rate per second
    let backlog = 128; // Number of pending connections
    let max_payload_size = 10 * 1024 * 1024; // Maximum payload size in bytes

    // Initialize shared application state
    let data = web::Data::new(Arc::new(AppState {
        connection_count: Mutex::new(0),
    }));

    // Log the server startup message
    info!("Starting server at {}", server_address);

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Compress::default())
            .service(web::resource("/").to(index))
            .service(web::resource("/health").to(health_check))
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
