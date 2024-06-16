mod analytics;

use analytics::data_analysis::analyze_data;
use analytics::real_time_processing::{start_real_time_processing, create_record_batch};

fn main() {
    // Example JSON data
    let json_data = r#"
    {
        "name": "NeuraServer",
        "status": "running",
        "uptime": 12345
    }
    "#;

    // Analyze data
    analyze_data(json_data);

    // Start real-time processing
    let (tx, _rx) = start_real_time_processing();

    // Create a record batch and send it for processing
    let batch = create_record_batch(json_data);
    tx.send(batch).unwrap();
}
