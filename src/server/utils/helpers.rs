use std::time::Duration;
use std::thread;

pub fn delay(seconds: u64) {
    thread::sleep(Duration::from_secs(seconds));
}

pub fn parse_input(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}
