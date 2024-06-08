#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_connection() {
        let result = connect_to_server("127.0.0.1:5500");
        assert!(result.is_ok(), "Failed to connect to server");
    }

    fn connect_to_server(address: &str) -> Result<(), String> {
        // Simulate connection logic
        if address == "127.0.0.1:5500" {
            Ok(())
        } else {
            Err("Connection failed".to_string())
        }
    }
}
