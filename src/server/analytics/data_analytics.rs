use arrow::array::{Float64Array, Int64Array, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::json::reader::Decoder;
use arrow::record_batch::RecordBatch;
use arrow::util::pretty::pretty_format_batches;
use serde_json::Value;
use std::sync::Arc;

pub fn analyze_data(json_data: &str) {
    let data: Value = serde_json::from_str(json_data).unwrap();

    // Define the schema for the data
    let schema = Arc::new(Schema::new(vec![
        Field::new("name", DataType::Utf8, false),
        Field::new("status", DataType::Utf8, false),
        Field::new("uptime", DataType::Int64, false),
    ]));

    // Create Arrow arrays
    let name_array = StringArray::from(vec![data["name"].as_str().unwrap()]);
    let status_array = StringArray::from(vec![data["status"].as_str().unwrap()]);
    let uptime_array = Int64Array::from(vec![data["uptime"].as_i64().unwrap()]);

    // Create a record batch
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(name_array) as Arc<dyn arrow::array::Array>,
            Arc::new(status_array),
            Arc::new(uptime_array),
        ],
    )
    .unwrap();

    // Print the batch
    let formatted = pretty_format_batches(&[batch]).unwrap();
    println!("Analyzing data:\n{}", formatted);
}
