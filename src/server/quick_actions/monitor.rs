use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn live_monitor(running: Arc<Mutex<bool>>) {
    thread::spawn(move || {
        while *running.lock().unwrap() {
            thread::sleep(Duration::from_secs(1));
            println!("Live monitoring the server status...");
        }
    });
}
