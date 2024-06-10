use rdkafka::config::ClientConfig;
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::message::Message;
use rdkafka::producer::{BaseProducer, BaseRecord};
use std::time::Duration;
use zookeeper::{Acl, CreateMode, WatchedEvent, Watcher, ZooKeeper};

struct LoggingWatcher;

impl Watcher for LoggingWatcher {
    fn handle(&self, event: WatchedEvent) {
        println!("Received ZooKeeper event: {:?}", event);
    }
}

#[tokio::main]
async fn main() {
    // Connect to ZooKeeper
    let zookeeper_servers = "127.0.0.1:2181";
    let zk = ZooKeeper::connect(zookeeper_servers, Duration::from_secs(10), LoggingWatcher).unwrap();

    // Create a znode
    let path = "/neuraserver";
    zk.create(path, vec![1, 2, 3, 4], Acl::open_unsafe().clone(), CreateMode::Persistent)
        .unwrap_or_else(|_| {
            println!("Znode {} already exists", path);
        });

    // Set up Kafka producer
    let producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", "127.0.0.1:9092")
        .create()
        .expect("Producer creation error");

    let topic = "neura_topic";
    let message = "Hello, this is a message from NeuraServer!";

    producer
        .send(BaseRecord::to(topic).payload(message))
        .expect("Failed to send message");

    producer.flush(Duration::from_secs(1));

    // Set up Kafka consumer
    let consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", "127.0.0.1:9092")
        .set("group.id", "neura_group")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Consumer creation error");

    consumer.subscribe(&[topic]).unwrap();

    loop {
        match consumer.poll(Duration::from_millis(100)) {
            Some(Ok(message)) => {
                let payload = message.payload().map(|s| String::from_utf8_lossy(s).to_string());
                println!("Received message: {:?}", payload);
            }
            Some(Err(e)) => println!("Kafka error: {}", e),
            None => (),
        }
    }
}
