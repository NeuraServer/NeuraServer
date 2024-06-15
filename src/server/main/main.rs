use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;
use log::{info, warn, error};
use simplelog::*;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    server_address: String,
    database_url: String,
}

#[derive(Debug, Serialize)]
struct ApiResponse {
    message: String,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

struct AppState {
    config: Config,
    data: Mutex<Vec<String>>,
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse {
        message: "Server is running".to_string(),
    })
}

async fn get_data(data: web::Data<AppState>) -> impl Responder {
    let data = data.data.lock().unwrap();
    HttpResponse::Ok().json(&*data)
}

async fn add_data(item: web::Json<String>, data: web::Data<AppState>) -> impl Responder {
    let mut data = data.data.lock().unwrap();
    data.push(item.into_inner());
    HttpResponse::Ok().json(ApiResponse {
        message: "Data added".to_string(),
    })
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_data = fs::read_to_string("config.json")?;
    let config: Config = serde_json::from_str(&config_data)?;
    Ok(config)
}

fn setup_logging() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, ConfigBuilder::new().set_time_to_local(true).build(), TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(LevelFilter::Info, ConfigBuilder::new().set_time_to_local(true).build(), fs::File::create("neuraserver.log").unwrap()),
    ]).unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_logging();

    let config = match load_config() {
        Ok(cfg) => {
            info!("Configuration loaded successfully");
            cfg
        }
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to load configuration"));
        }
    };

    let data = web::Data::new(AppState {
        config: config.clone(),
        data: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/health", web::get().to(health_check))
            .route("/data", web::get().to(get_data))
            .route("/data", web::post().to(add_data))
    })
    .bind(&config.server_address)?
    .run()
    .await
}
