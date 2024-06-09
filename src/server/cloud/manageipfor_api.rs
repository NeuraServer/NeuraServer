use actix_web::{web, App, HttpServer, HttpResponse, Responder, HttpRequest};
use jsonwebtoken::{encode, Header, EncodingKey, Algorithm};
use serde::{Deserialize, Serialize};
use std::env;
use oauth2::{basic::BasicClient, AuthUrl, TokenUrl, AuthorizationCode, RedirectUrl, Scope};

#[derive(Serialize, Deserialize)]
struct ApiKeyRequest {
    address: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    address: String,
    exp: usize,
}

async fn generate_api_key(info: web::Json<ApiKeyRequest>) -> impl Responder {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();
    let claims = Claims {
        address: info.address.clone(),
        exp: expiration as usize,
    };

    let key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(key.as_ref())).unwrap();
    HttpResponse::Ok().json(serde_json::json!({ "apiKey": token }))
}

async fn start_oauth2(req: HttpRequest) -> impl Responder {
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");
    let auth_url = env::var("AUTH_URL").expect("AUTH_URL must be set");
    let token_url = env::var("TOKEN_URL").expect("TOKEN_URL must be set");
    let redirect_url = env::var("REDIRECT_URL").expect("REDIRECT_URL must be set");

    let client = BasicClient::new(
        client_id.into(),
        Some(client_secret.into()),
        AuthUrl::new(auth_url).unwrap(),
        Some(TokenUrl::new(token_url).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap());

    let (auth_url, _csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("read".into()))
        .add_scope(Scope::new("write".into()))
        .url();

    HttpResponse::Found()
        .header("Location", auth_url.to_string())
        .finish()
}

async fn handle_oauth2_callback(req: HttpRequest) -> impl Responder {
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");
    let token_url = env::var("TOKEN_URL").expect("TOKEN_URL must be set");
    let redirect_url = env::var("REDIRECT_URL").expect("REDIRECT_URL must be set");

    let client = BasicClient::new(
        client_id.into(),
        Some(client_secret.into()),
        AuthUrl::new(auth_url).unwrap(),
        Some(TokenUrl::new(token_url).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap());

    let query = req.query_string();
    let code = query.split('&').find_map(|s| {
        let mut split = s.split('=');
        match (split.next(), split.next()) {
            (Some("code"), Some(code)) => Some(code.to_string()),
            _ => None,
        }
    });

    if let Some(code) = code {
        let token_result = client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(oauth2::reqwest::async_http_client)
            .await;

        match token_result {
            Ok(token) => HttpResponse::Ok().json(token),
            Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
        }
    } else {
        HttpResponse::BadRequest().body("No code found in query string")
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .route("/generate-api-key", web::post().to(generate_api_key))
            .route("/oauth2", web::get().to(start_oauth2))
            .route("/callback", web::get().to(handle_oauth2_callback))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
