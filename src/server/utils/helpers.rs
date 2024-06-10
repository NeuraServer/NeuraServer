use std::time::Duration;
use std::thread;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub fn delay(seconds: u64) {
    thread::sleep(Duration::from_secs(seconds));
}

pub fn parse_input(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}

pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

pub fn get_current_timestamp() -> DateTime<Utc> {
    Utc::now()
}

pub fn retry_operation<F, T>(mut operation: F, retries: usize) -> Option<T>
where
    F: FnMut() -> Option<T>,
{
    for _ in 0..retries {
        if let Some(result) = operation() {
            return Some(result);
        }
        thread::sleep(Duration::from_secs(1));
    }
    None
}
