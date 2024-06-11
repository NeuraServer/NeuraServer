use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::sync::{Arc, Mutex};

struct ProxyState {
    request_count: Mutex<i64>,
}

async fn manage_proxy(data: web::Data<Arc<ProxyState>>) -> impl Responder {
    let mut count = data.request_count.lock().unwrap();
    *count += 1;
    HttpResponse::Ok().body(format!("Proxy request number: {}", count))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let server_address = "127.0.0.1:5500";
    let data = web::Data::new(Arc::new(ProxyState {
        request_count: Mutex::new(0),
    }));

    println!("Starting proxy server at {}", server_address);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Compress::default())
            .service(web::resource("/proxy").to(manage_proxy))
    })
    .bind(server_address)?
    .run()
    .await
}
