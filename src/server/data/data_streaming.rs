use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::sync::{Arc, Mutex};
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::ClientConfig;
use futures::StreamExt;

struct AppState {
    kafka_consumer: Arc<StreamConsumer>,
}

async fn stream_data(data: web::Data<Arc<AppState>>) -> impl Responder {
    let mut stream = data.kafka_consumer.start();
    let mut messages = Vec::new();

    while let Some(message) = stream.next().await {
        match message {
            Ok(m) => {
                if let Some(payload) = m.payload_view::<str>().unwrap() {
                    messages.push(payload.to_string());
                }
            }
            Err(e) => {
                eprintln!("Error receiving message: {:?}", e);
                break;
            }
        }
    }

    HttpResponse::Ok().json(messages)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "my-group")
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(&["my_topic"]).expect("Failed to subscribe");

    let data = web::Data::new(Arc::new(AppState { kafka_consumer: Arc::new(consumer) }));

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(web::resource("/stream_data").to(stream_data))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
