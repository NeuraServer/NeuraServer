use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
    email: String,
}

async fn validate_user(info: web::Json<Info>) -> impl Responder {
    if info.username.is_empty() || info.email.is_empty() {
        return HttpResponse::BadRequest().body("Invalid input");
    }

    HttpResponse::Ok().body("Valid input")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/validate", web::post().to(validate_user))
    })
    .bind("127.0.0.1:5500")?
    .run()
    .await
}
