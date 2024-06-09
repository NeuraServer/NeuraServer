use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::sync::{Arc, Mutex};
use sqlx::postgres::PgPoolOptions;
use dashmap::DashMap;
use chrono::{Utc, Duration};

struct AppState {
    connection_count: Mutex<i64>,
    db_pool: sqlx::PgPool,
    cache: Arc<DashMap<String, (String, i64)>>,
}

async fn index(data: web::Data<Arc<AppState>>) -> impl Responder {
    let mut count = data.connection_count.lock().unwrap();
    *count += 1;
    let timestamp = Utc::now().timestamp();
    data.cache.insert("last_visitor".to_string(), (format!("Visitor number: {}", count), timestamp));
    HttpResponse::Ok().body(format!("Hello, visitor number: {}", count))
}

async fn cache_cleaner(data: web::Data<Arc<AppState>>) {
    let expiration_time = 60; // 1 minute
    loop {
        let now = Utc::now().timestamp();
        data.cache.retain(|_, (_, timestamp)| now - *timestamp <= expiration_time);
        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await; // Clean cache every 30 seconds
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    let data = web::Data::new(Arc::new(AppState {
        connection_count: Mutex::new(0),
        db_pool,
        cache: Arc::new(DashMap::new()),
    }));

    let cache_data = data.clone();
    tokio::spawn(async move {
        cache_cleaner(cache_data).await;
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Compress::default())
            .service(web::resource("/").to(index))
            .service(web::resource("/health").to(|| HttpResponse::Ok().body("Server is running")))
    })
    .workers(8)
    .max_connections(20_000)
    .bind("127.0.0.1:5500")?
    .run()
    .await
}
