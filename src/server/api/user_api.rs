use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

struct AppState {
    users: Mutex<Vec<User>>,
}

async fn get_users(data: web::Data<AppState>) -> impl Responder {
    let users = data.users.lock().unwrap();
    HttpResponse::Ok().json(&*users)
}

async fn get_user(data: web::Data<AppState>, user_id: web::Path<u32>) -> impl Responder {
    let users = data.users.lock().unwrap();
    if let Some(user) = users.iter().find(|u| u.id == *user_id) {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

async fn create_user(data: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    users.push(user.into_inner());
    HttpResponse::Created().body("User created")
}

async fn delete_user(data: web::Data<AppState>, user_id: web::Path<u32>) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    if let Some(pos) = users.iter().position(|u| u.id == *user_id) {
        users.remove(pos);
        HttpResponse::Ok().body("User deleted")
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let users = web::Data::new(AppState {
        users: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(users.clone())
            .route("/users", web::get().to(get_users))
            .route("/users/{id}", web::get().to(get_user))
            .route("/users", web::post().to(create_user))
            .route("/users/{id}", web::delete().to(delete_user))
    })
    .bind("127.0.0.1:5500")?
    .run()
    .await
}
