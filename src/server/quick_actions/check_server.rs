use std::net::TcpStream;
use std::time::Duration;

pub fn check_server(ip: &str, port: u16) {
    let address = format!("{}:{}", ip, port);
    match TcpStream::connect_timeout(&address.parse().unwrap(), Duration::from_secs(2)) {
        Ok(_) => println!("Server {} is UP", address),
        Err(e) => println!("Server {} is DOWN: {}", address, e),
    }
}
