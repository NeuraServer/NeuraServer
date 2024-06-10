use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::message::Message;
use rdkafka::producer::{BaseProducer, BaseRecord};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::Duration;
use zookeeper::{Acl, CreateMode, WatchedEvent, Watcher, ZooKeeper};

struct AppState {
    producer: Mutex<BaseProducer>,
    consumer: Mutex<BaseConsumer>,
    zk: Mutex<ZooKeeper>,
}

struct LoggingWatcher;

impl Watcher for LoggingWatcher {
    fn handle(&self, event: WatchedEvent) {
        println!("Received ZooKeeper event: {:?}", event);
    }
}

#[derive(Deserialize)]
struct ProduceRequest {
    topic: String,
    message: String,
}

#[derive(Serialize)]
struct ConsumeResponse {
    message: String,
}

async fn produce(
    data: web::Data<AppState>,
    req: web::Json<ProduceRequest>,
) -> impl Responder {
    let producer = data.producer.lock().unwrap();
    producer
        .send(BaseRecord::to(&req.topic).payload(&req.message))
        .expect("Failed to send message");
    producer.flush(Duration::from_secs(1));
    HttpResponse::Ok().body("Message produced successfully")
}

async fn consume(
    data: web::Data<AppState>,
    web::Query((topic,)): web::Query<(String,)>,
) -> impl Responder {
    let consumer = data.consumer.lock().unwrap();
    consumer.subscribe(&[&topic]).unwrap();

    let msg = consumer.poll(Duration::from_secs(1)).unwrap();
    match msg {
        Some(Ok(message)) => {
            let payload = message.payload().map(|s| String::from_utf8_lossy(s).to_string());
            HttpResponse::Ok().json(ConsumeResponse {
                message: payload.unwrap_or_default(),
            })
        }
        Some(Err(e)) => HttpResponse::InternalServerError().body(format!("Kafka error: {}", e)),
        None => HttpResponse::NotFound().body("No message found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Connect to ZooKeeper
    let zookeeper_servers = "127.0.0.1:2181";
    let zk = ZooKeeper::connect(zookeeper_servers, Duration::from_secs(10), LoggingWatcher)
        .expect("Failed to connect to ZooKeeper");

    // Create Kafka producer
    let producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", "127.0.0.1:9092")
        .create()
        .expect("Producer creation error");

    // Create Kafka consumer
    let consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", "127.0.0.1:9092")
        .set("group.id", "neura_group")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Consumer creation error");

    let data = web::Data::new(AppState {
        producer: Mutex::new(producer),
        consumer: Mutex::new(consumer),
        zk: Mutex::new(zk),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/produce", web::post().to(produce))
            .route("/consume", web::get().to(consume))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
