use actix_web::{web, HttpResponse};

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json("Server is running")
}
