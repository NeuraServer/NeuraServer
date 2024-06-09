use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

async fn login() -> impl Responder {
    let claims = Claims {
        sub: "user1".to_owned(),
        exp: 10000000000,
    };

    let key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(key.as_ref())).unwrap();
    HttpResponse::Ok().json(token)
}

async fn validate_token(token: String) -> impl Responder {
    let key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let token_data = decode::<Claims>(&token, &DecodingKey::from_secret(key.as_ref()), &Validation::default()).unwrap();
    HttpResponse::Ok().json(token_data.claims)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .route("/login", web::get().to(login))
            .route("/validate", web::post().to(validate_token))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
