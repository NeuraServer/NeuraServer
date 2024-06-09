use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::sync::{Arc, Mutex};
use redis::AsyncCommands;
use actix_web::middleware::Logger;
use log::info;

struct AppState {
    connection_count: Mutex<i64>,
}

async fn index(data: web::Data<Arc<AppState>>) -> impl Responder {
    let mut count = data.connection_count.lock().unwrap();
    *count += 1;
    HttpResponse::Ok().body(format!("Hello, visitor number: {}", count))
}

async fn get_redis_value() -> impl Responder {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_async_connection().await.unwrap();
    let result: String = con.get("key").await.unwrap();
    HttpResponse::Ok().body(result)
}

async fn set_redis_value() -> impl Responder {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_async_connection().await.unwrap();
    let _: () = con.set("key", "value").await.unwrap();
    HttpResponse::Ok().body("Value set in Redis")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(Arc::new(AppState {
        connection_count: Mutex::new(0),
    }));

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(Logger::default())
            .service(web::resource("/").to(index))
            .service(web::resource("/redis/get").to(get_redis_value))
            .service(web::resource("/redis/set").to(set_redis_value))
    })
    .bind("127.0.0.1:5500")?
    .run()
    .await
}
