use actix_session::{CookieSession, Session};
use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest, Responder};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    last_login: u64,
}

async fn login(session: Session, user: web::Json<User>) -> impl Responder {
    let login_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let user_data = User {
        username: user.username.clone(),
        last_login: login_time,
    };
    session.insert("user", &user_data).unwrap();
    HttpResponse::Ok().json("Login successful")
}

async fn get_session_info(session: Session) -> impl Responder {
    if let Some(user) = session.get::<User>("user").unwrap() {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::Ok().json("No user logged in")
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .route("/login", web::post().to(login))
            .route("/session", web::get().to(get_session_info))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
