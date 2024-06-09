use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct LoginInfo {
    username: String,
    password: String,
}

async fn login(info: web::Json<LoginInfo>) -> impl Responder {
    if info.username == "admin" && info.password == "password" {
        HttpResponse::Ok().body("Login successful")
    } else {
        HttpResponse::Unauthorized().body("Invalid credentials")
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/login").route(web::post().to(login)));
}
