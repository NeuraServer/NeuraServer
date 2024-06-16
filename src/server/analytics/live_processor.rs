use arrow::array::{Float64Array, Int64Array, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use arrow::util::pretty::pretty_format_batches;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn start_real_time_processing() -> (Sender<RecordBatch>, Receiver<RecordBatch>) {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        while let Ok(batch) = rx.recv() {
            // Simulate real-time processing
            let formatted = pretty_format_batches(&[batch]).unwrap();
            println!("Processing data:\n{}", formatted);
            thread::sleep(Duration::from_secs(1));
        }
    });

    (tx, rx)
}

pub fn create_record_batch(data: &str) -> RecordBatch {
    let data: serde_json::Value = serde_json::from_str(data).unwrap();

    let schema = Arc::new(Schema::new(vec![
        Field::new("name", DataType::Utf8, false),
        Field::new("status", DataType::Utf8, false),
        Field::new("uptime", DataType::Int64, false),
    ]));

    let name_array = StringArray::from(vec![data["name"].as_str().unwrap()]);
    let status_array = StringArray::from(vec![data["status"].as_str().unwrap()]);
    let uptime_array = Int64Array::from(vec![data["uptime"].as_i64().unwrap()]);

    RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(name_array) as Arc<dyn arrow::array::Array>,
            Arc::new(status_array),
            Arc::new(uptime_array),
        ],
    )
    .unwrap()
}
