use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs;

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
    state: HashMap<String, String>,
}

impl Blockchain {
    fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            current_data: Vec::new(),
            state: HashMap::new(),
        };
        blockchain.chain.push(Block::new(0, 0, String::from("0"), String::new()));
        blockchain
    }

    fn add_block(&mut self) {
        let previous_block = self.chain.last().unwrap();
        let block = Block::new(
            previous_block.index + 1,
            get_current_time(),
            previous_block.hash.clone(),
            self.current_data.join(","),
        );
        self.current_data.clear();
        self.chain.push(block);
    }

    fn add_data(&mut self, data: String) {
        self.current_data.push(data);
    }

    fn save_state(&self) {
        let serialized = serde_json::to_string(&self.state).unwrap();
        fs::write("blockchain_state.json", serialized).unwrap();
    }

    fn load_state(&mut self) {
        let data = fs::read_to_string("blockchain_state.json").unwrap();
        self.state = serde_json::from_str(&data).unwrap();
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
