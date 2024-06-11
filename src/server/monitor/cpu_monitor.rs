use sysinfo::{ProcessorExt, System, SystemExt};
use std::net::TcpStream;
use std::io::{Write, Read};
use std::thread;
use std::time::Duration;

fn log_cpu_usage(stream: &mut TcpStream, usage: f32) -> Result<(), std::io::Error> {
    let message = format!("CPU Usage: {:.2}%\n", usage);
    stream.write_all(message.as_bytes())?;
    Ok(())
}

fn main() {
    let mut system = System::new_all();
    let mut stream = match TcpStream::connect("127.0.0.1:5500") {
        Ok(stream) => stream,
        Err(err) => {
            eprintln!("Error: Could not connect to server: {}", err);
            return;
        }
    };

    loop {
        if let Err(err) = system.refresh_all() {
            eprintln!("Error: Failed to refresh system data: {}", err);
            continue;
        }

        let cpu_usage = match system.global_processor_info().cpu_usage() {
            Some(usage) => usage,
            None => {
                eprintln!("Error: Failed to retrieve CPU usage");
                continue;
            }
        };

        println!("CPU Usage: {:.2}%", cpu_usage);
        if let Err(err) = log_cpu_usage(&mut stream, cpu_usage) {
            eprintln!("Error: Failed to log CPU usage: {}", err);
            continue;
        }

        thread::sleep(Duration::from_secs(1));
    }
}
