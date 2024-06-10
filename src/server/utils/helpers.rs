use std::time::Duration;
use std::thread;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::fs::File;
use std::io::{self, Read, Write};

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

pub fn read_file_to_string(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn write_string_to_file(path: &str, contents: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())
}

pub fn calculate_average(numbers: &[f64]) -> f64 {
    let sum: f64 = numbers.iter().sum();
    sum / numbers.len() as f64
}

pub fn get_file_extension(filename: &str) -> Option<&str> {
    filename.split('.').last()
}

pub fn is_palindrome(s: &str) -> bool {
    let filtered: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
    let reversed: String = filtered.chars().rev().collect();
    filtered.eq_ignore_ascii_case(&reversed)
}
