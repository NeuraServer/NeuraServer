use std::net::TcpStream;
use std::io::{Write, Read};
use std::thread;
use std::time::Duration;
use std::fs::OpenOptions;

fn log_security_event(stream: &mut TcpStream, event: &str) {
    let message = format!("Security Event: {}\n", event);
    stream.write_all(message.as_bytes()).unwrap();
}

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:5500").expect("Could not connect to server");
    let events = vec![
        "Login attempt from IP 192.168.1.1",
        "Failed password attempt for user admin",
        "New SSH key added for user deploy",
    ];

    for event in events {
        println!("Security Event: {}", event);
        log_security_event(&mut stream, event);
        thread::sleep(Duration::from_secs(5));
    }
}
