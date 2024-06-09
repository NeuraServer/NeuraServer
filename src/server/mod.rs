// ----------------------------------
// --- NeuraServer Copyright 2024 ---
// ----------------------------------

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::sync::{Arc, Mutex};
use std::env;
use dotenv::dotenv;
use log::info;
use env_logger;

struct AppState {
    connection_count: Mutex<i64>,
    request_count: Mutex<i64>,
}

async fn index(data: web::Data<Arc<AppState>>) -> impl Responder {
    let mut conn_count = data.connection_count.lock().unwrap();
    let mut req_count = data.request_count.lock().unwrap();
    *conn_count += 1;
    *req_count += 1;
    HttpResponse::Ok().body(format!("Hello, visitor number: {}. Total requests: {}", conn_count, req_count))
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is running")
}

async fn stats(data: web::Data<Arc<AppState>>) -> impl Responder {
    let conn_count = data.connection_count.lock().unwrap();
    let req_count = data.request_count.lock().unwrap();
    HttpResponse::Ok().body(format!("Total connections: {}. Total requests: {}", conn_count, req_count))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "5500".to_string());
    let server_address = format!("{}:{}", host, port);
    let workers = 4;
    let max_connections = 10_000;
    let max_connection_rate = 256;
    let backlog = 128;
    let max_payload_size = 10 * 1024 * 1024;
    let data = web::Data::new(Arc::new(AppState {
        connection_count: Mutex::new(0),
        request_count: Mutex::new(0),
    }));
    info!("Starting server at {}", server_address);
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Compress::default())
            .service(web::resource("/").to(index))
            .service(web::resource("/health").to(health_check))
            .service(web::resource("/stats").to(stats))
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
