use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::sync::{Arc, Mutex};
use tiberius::{Client, Config, AuthMethod};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use futures::stream::StreamExt;

struct AppState {
    client: Mutex<Client<TcpStream>>,
    allowed_tables: Mutex<Vec<String>>,
}

async fn get_user(data: web::Data<Arc<AppState>>, id: web::Path<i32>) -> impl Responder {
    let mut client = data.client.lock().unwrap();
    let allowed_tables = data.allowed_tables.lock().unwrap();

    if !allowed_tables.contains(&"users".to_string()) {
        return HttpResponse::Forbidden().body("Access denied");
    }

    let query = format!("SELECT name FROM users WHERE id = {}", id);
    let result = client.simple_query(query).await;
    
    match result {
        Ok(mut row) => {
            let name: &str = row.next().await.unwrap().unwrap().get(0).unwrap();
            HttpResponse::Ok().body(format!("User: {}", name))
        },
        Err(_) => HttpResponse::InternalServerError().body("Error querying the database"),
    }
}

async fn set_user(data: web::Data<Arc<AppState>>, info: web::Json<(i32, String)>) -> impl Responder {
    let mut client = data.client.lock().unwrap();
    let (id, name) = info.into_inner();

    let query = format!("INSERT INTO users (id, name) VALUES ({}, '{}')", id, name);
    let result = client.simple_query(query).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("User added"),
        Err(_) => HttpResponse::InternalServerError().body("Error inserting into the database"),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut config = Config::new();
    config.host("127.0.0.1");
    config.port(1433);
    config.authentication(AuthMethod::sql_server("SA", "your_password"));

    let tcp = TcpStream::connect(config.get_addr()).await.unwrap();
    tcp.set_nodelay(true).unwrap();
    let client = Client::connect(config, tcp.compat_write()).await.unwrap();

    let data = web::Data::new(Arc::new(AppState {
        client: Mutex::new(client),
        allowed_tables: Mutex::new(vec!["users".to_string()]),
    }));

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Compress::default())
            .service(web::resource("/user/{id}").to(get_user))
            .service(web::resource("/user").route(web::post().to(set_user)))
    })
    .bind("127.0.0.1:5500")?
    .run()
    .await
}
