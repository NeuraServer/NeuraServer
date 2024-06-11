use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};

struct RateLimiterState {
    request_count: Mutex<i64>,
    last_request: Mutex<Instant>,
}

async fn rate_limit(data: web::Data<Arc<RateLimiterState>>) -> impl Responder {
    let mut count = data.request_count.lock().unwrap();
    let mut last_request = data.last_request.lock().unwrap();

    if last_request.elapsed() < Duration::from_secs(1) {
        HttpResponse::TooManyRequests().body("Rate limit exceeded")
    } else {
        *count += 1;
        *last_request = Instant::now();
        HttpResponse::Ok().body(format!("Rate limit request number: {}", count))
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let server_address = "127.0.0.1:5500";
    let data = web::Data::new(Arc::new(RateLimiterState {
        request_count: Mutex::new(0),
        last_request: Mutex::new(Instant::now()),
    }));

    println!("Starting rate limiter server at {}", server_address);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Compress::default())
            .service(web::resource("/ratelimit").to(rate_limit))
    })
    .bind(server_address)?
    .run()
    .await
}
