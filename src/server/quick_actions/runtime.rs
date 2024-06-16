mod quick_actions;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use quick_actions::check_server::check_server;
use quick_actions::live_monitor::live_monitor;

fn main() {
    let running = Arc::new(Mutex::new(true));
    let monitor_running = running.clone();

    // Start live monitoring in a separate thread
    live_monitor(monitor_running);

    // Check server status
    let ip = "127.0.0.1";
    let port = 8080;
    check_server(ip, port);

    // Simulate running for a certain duration
    thread::sleep(Duration::from_secs(10));

    // Stop the live monitor
    let mut running = running.lock().unwrap();
    *running = false;
}
