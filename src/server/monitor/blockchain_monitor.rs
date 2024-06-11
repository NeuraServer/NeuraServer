use serde::{Serialize, Deserialize};
use std::net::TcpStream;
use std::io::{Write, Read};
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;

#[derive(Serialize, Deserialize, Debug)]
struct Block {
    index: u64,
    timestamp: u128,
    data: String,
    prev_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u64, data: String, prev_hash: String) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        let hash = format!("{:x}", md5::compute(format!("{}{}{}{}", index, timestamp, &data, &prev_hash)));
        Block {
            index,
            timestamp,
            data,
            prev_hash,
            hash,
        }
    }
}

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:5500").expect("Could not connect to server");
    let mut blockchain = vec![];

    for i in 0..10 {
        let prev_hash = if blockchain.is_empty() {
            String::from("0")
        } else {
            blockchain.last().unwrap().hash.clone()
        };
        let block = Block::new(i, format!("Block {}", i), prev_hash);
        blockchain.push(block.clone());

        let message = serde_json::to_string(&block).unwrap();
        stream.write_all(message.as_bytes()).unwrap();
        thread::sleep(std::time::Duration::from_secs(1));
    }
}
