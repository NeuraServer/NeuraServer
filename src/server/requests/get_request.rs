use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct RequestData {
    message: String,
}

async fn receive_get_request() -> impl Responder {
    HttpResponse::Ok().body("Received a GET request")
}

async fn receive_post_request(data: web::Json<RequestData>) -> impl Responder {
    HttpResponse::Ok().json(RequestData {
        message: format!("Received a POST request with message: {}", data.message),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/receive_get_request", web::get().to(receive_get_request))
            .route("/receive_post_request", web::post().to(receive_post_request))
    })
    .bind("127.0.0.1:5500")?
    .run()
    .await
}
