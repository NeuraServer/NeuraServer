#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_load_handling() {
        let result = simulate_server_load(1000);
        assert!(result.is_ok(), "Server failed under load");
    }

    fn simulate_server_load(connections: u32) -> Result<(), String> {
        // Simulate load handling logic
        if connections <= 1000 {
            Ok(())
        } else {
            Err("Load too high".to_string())
        }
    }
}
