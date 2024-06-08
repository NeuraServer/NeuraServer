// ----------------------------------
// --- NeuraServer Copyright 2024 ---
// ----------------------------------

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::sync::{Arc, Mutex};

struct AppState {
    connection_count: Mutex<i32>,
}

async fn index(data: web::Data<Arc<AppState>>) -> impl Responder {
    let mut count = data.connection_count.lock().unwrap();
    *count += 1;
    HttpResponse::Ok().body(format!("Hello, visitor number: {}", count))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Configuration options
    let server_address = "127.0.0.1:5500";
    let workers = 4; // Number of worker threads
    let max_connections = 10_000; // Maximum number of concurrent connections
    let max_connection_rate = 256; // Maximum connection rate per second
    let backlog = 128; // Number of pending connections
    let max_payload_size = 10 * 1024 * 1024; // Maximum payload size in bytes

    let data = web::Data::new(Arc::new(AppState {
        connection_count: Mutex::new(0),
    }));

    println!("Starting server at {}", server_address);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Compress::default())
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

