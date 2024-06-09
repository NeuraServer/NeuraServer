use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use tokio::task;
use redis::AsyncCommands;
use tokio::time::{sleep, Duration};

async fn process_task(task_id: String) {
    sleep(Duration::from_secs(5)).await;
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_async_connection().await.unwrap();
    let _: () = con.lrem("task_queue", 1, task_id).await.unwrap();
}

async fn add_task() -> impl Responder {
    let task_id = "task_1".to_string();
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_async_connection().await.unwrap();
    let _: () = con.lpush("task_queue", &task_id).await.unwrap();

    task::spawn(process_task(task_id));
    HttpResponse::Ok().body("Task added")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/add_task").to(add_task))
    })
    .bind("127.0.0.1:5500")?
    .run()
    .await
}
