use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use reqwest;

#[derive(Debug, Serialize, Deserialize)]
struct ResponseData {
    message: String,
}

async fn send_get_request() -> impl Responder {
    let response = reqwest::get("https://jsonplaceholder.typicode.com/posts/1")
        .await
        .unwrap()
        .json::<ResponseData>()
        .await
        .unwrap();

    HttpResponse::Ok().json(response)
}

async fn send_post_request() -> impl Responder {
    let client = reqwest::Client::new();
    let response = client
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&ResponseData {
            message: String::from("Hello, World!"),
        })
        .send()
        .await
        .unwrap()
        .json::<ResponseData>()
        .await
        .unwrap();

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/send_get_request", web::get().to(send_get_request))
            .route("/send_post_request", web::post().to(send_post_request))
    })
    .bind("127.0.0.1:5500")?
    .run()
    .await
}
