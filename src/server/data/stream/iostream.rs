use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tokio::runtime::Runtime;
use hdfs::Client;

struct AppState {
    hdfs_client: Mutex<Client>,
}

#[derive(Deserialize)]
struct HDFSRequest {
    path: String,
}

#[derive(Serialize)]
struct HDFSResponse {
    file_content: String,
}

async fn read_hdfs(data: web::Data<AppState>, req: web::Json<HDFSRequest>) -> impl Responder {
    let client = data.hdfs_client.lock().unwrap();
    match client.open(&req.path) {
        Ok(mut file) => {
            let mut content = String::new();
            if let Ok(_) = file.read_to_string(&mut content) {
                HttpResponse::Ok().json(HDFSResponse { file_content: content })
            } else {
                HttpResponse::InternalServerError().body("Failed to read HDFS file")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to open HDFS file"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let rt = Runtime::new().unwrap();
    let hdfs_client = rt.block_on(async {
        Client::new("hdfs://namenode:9000").await.expect("Failed to connect to HDFS")
    });

    let data = web::Data::new(AppState {
        hdfs_client: Mutex::new(hdfs_client),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/read_hdfs", web::post().to(read_hdfs))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
