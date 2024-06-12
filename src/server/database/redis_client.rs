use redis::{Client, Commands, RedisResult};
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::sync::{Arc, Mutex};

struct AppState {
    redis_client: Mutex<Client>,
    allowed_keys: Mutex<Vec<String>>,
}

async fn get_value(data: web::Data<Arc<AppState>>, key: web::Path<String>) -> impl Responder {
    let client = data.redis_client.lock().unwrap();
    let allowed_keys = data.allowed_keys.lock().unwrap();
    
    if !allowed_keys.contains(&key.into_inner()) {
        return HttpResponse::Forbidden().body("Access denied");
    }

    let mut con = client.get_connection().unwrap();
    let value: RedisResult<String> = con.get(&*key);
    match value {
        Ok(val) => HttpResponse::Ok().body(val),
        Err(_) => HttpResponse::NotFound().body("Key not found"),
    }
}

async fn set_value(data: web::Data<Arc<AppState>>, info: web::Json<(String, String)>) -> impl Responder {
    let client = data.redis_client.lock().unwrap();
    let (key, value) = info.into_inner();

    let mut con = client.get_connection().unwrap();
    let _ : RedisResult<()> = con.set(&key, value);

    HttpResponse::Ok().body("Value set")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let redis_client = Client::open("redis://127.0.0.1/").unwrap();
    let data = web::Data::new(Arc::new(AppState {
        redis_client: Mutex::new(redis_client),
        allowed_keys: Mutex::new(vec!["allowed_key".to_string()]),
    }));

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Compress::default())
            .service(web::resource("/get/{key}").to(get_value))
            .service(web::resource("/set").route(web::post().to(set_value)))
    })
    .bind("127.0.0.1:5500")?
    .run()
    .await
}
