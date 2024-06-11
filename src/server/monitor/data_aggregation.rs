use std::net::TcpStream;
use std::io::{Write, Read};
use std::thread;
use std::time::Duration;
use serde_json::Value;

fn send_aggregated_data(stream: &mut TcpStream, data: &str) {
    let message = format!("Aggregated Data: {}\n", data);
    stream.write_all(message.as_bytes()).unwrap();
}

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:5500").expect("Could not connect to server");
    let data_sources = vec![
        r#"{"sensor_id": "temp_sensor_1", "value": 22.5}"#,
        r#"{"sensor_id": "temp_sensor_2", "value": 23.0}"#,
        r#"{"sensor_id": "humidity_sensor_1", "value": 45.0}"#,
    ];

    let mut aggregated_data = vec![];

    for data in data_sources {
        let v: Value = serde_json::from_str(data).unwrap();
        aggregated_data.push(v);
    }

    let aggregated_json = serde_json::to_string(&aggregated_data).unwrap();
    println!("Aggregated Data: {}", aggregated_json);
    send_aggregated_data(&mut stream, &aggregated_json);
    thread::sleep(Duration::from_secs(10));
}
