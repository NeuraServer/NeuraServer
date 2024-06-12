use kafka::producer::{Producer, Record, RequiredAcks};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;

const KAFKA_BROKER: &str = "127.0.0.1:9092";
const TOPIC: &str = "data_pipeline";

fn main() {
    let mut producer = Producer::from_hosts(vec![KAFKA_BROKER.to_owned()])
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();

    let file = File::open("data/input.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let chunk = line.unwrap();
        producer.send(&Record::from_value(TOPIC, chunk)).unwrap();
        println!("Sent: {}", chunk);
    }
}
