use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Block {
    index: u64,
    timestamp: u64,
    previous_hash: String,
    hash: String,
    data: String,
}

struct Blockchain {
    chain: Vec<Block>,
    current_data: Vec<String>,
}

impl Blockchain {
    fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            current_data: Vec::new(),
        };
        blockchain.chain.push(Block::new(0, 0, String::from("0"), String::new()));
        blockchain
    }

    fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap();
        let block = Block::new(
            previous_block.index + 1,
            get_current_time(),
            previous_block.hash.clone(),
            data,
        );
        self.chain.push(block);
    }
}

fn get_current_time() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_secs()
}

impl Block {
    fn new(index: u64, timestamp: u64, previous_hash: String, data: String) -> Self {
        let hash = Block::calculate_hash(index, timestamp, &previous_hash, &data);
        Block {
            index,
            timestamp,
            previous_hash,
            hash,
            data,
        }
    }

    fn calculate_hash(index: u64, timestamp: u64, previous_hash: &str, data: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(index.to_string());
        hasher.update(timestamp.to_string());
        hasher.update(previous_hash);
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
}

fn handle_client(stream: TcpStream, blockchain: Arc<Mutex<Blockchain>>) {
    let mut buffer = [0; 512];
    let mut stream = stream;
    stream.read(&mut buffer).unwrap();
    let request: String = String::from_utf8_lossy(&buffer).trim_matches(char::from(0)).to_string();

    let block_data: String = request;
    let mut blockchain = blockchain.lock().unwrap();
    blockchain.add_block(block_data);

    let response = serde_json::to_string(&blockchain.chain).unwrap();
    stream.write(response.as_bytes()).unwrap();
}

fn main() {
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let blockchain = Arc::clone(&blockchain);
        std::thread::spawn(move || {
            handle_client(stream, blockchain);
        });
    }
}
