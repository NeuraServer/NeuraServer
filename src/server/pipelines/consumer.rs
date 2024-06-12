use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use std::time::Duration;
use std::fs::OpenOptions;
use std::io::Write;

const KAFKA_BROKER: &str = "127.0.0.1:9092";
const TOPIC: &str = "data_pipeline";
const GROUP_ID: &str = "data_pipeline_group";

fn main() {
    let mut consumer = Consumer::from_hosts(vec![KAFKA_BROKER.to_owned()])
        .with_topic(TOPIC.to_owned())
        .with_group(GROUP_ID.to_owned())
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()
        .unwrap();

    let mut file = OpenOptions::new().append(true).open("data/output.txt").unwrap();

    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                let chunk = String::from_utf8(m.value.to_vec()).unwrap();
                println!("Received: {}", chunk);
                writeln!(file, "{}", chunk).unwrap();
            }
            consumer.consume_messageset(ms).unwrap();
        }
        consumer.commit_consumed().unwrap();
        std::thread::sleep(Duration::from_secs(1)); // Polling interval
    }
}
