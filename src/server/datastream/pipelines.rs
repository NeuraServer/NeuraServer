use crate::data::processor::DataProcessor;
use tokio::task;
use std::sync::Arc;

pub struct DataPipeline {
    processor: Arc<DataProcessor>,
}

impl DataPipeline {
    pub fn new() -> Self {
        let processor = Arc::new(DataProcessor::new());
        Self { processor }
    }

    pub fn register_csv(&self, name: &str, path: &str) {
        let processor = self.processor.clone();
        task::spawn(async move {
            processor.register_csv(name, path).unwrap();
        });
    }

    pub fn register_parquet(&self, name: &str, path: &str) {
        let processor = self.processor.clone();
        task::spawn(async move {
            processor.register_parquet(name, path).unwrap();
        });
    }

    pub fn execute_query(&self, query: &str) {
        let processor = self.processor.clone();
        task::spawn(async move {
            let result = processor.execute_query(query).await;
            match result {
                Ok(batches) => {
                    for batch in batches {
                        println!("{:?}", batch);
                    }
                }
                Err(e) => eprintln!("Query execution failed: {:?}", e),
            }
        });
    }

    pub fn query_to_csv(&self, query: &str, output_path: &str) {
        let processor = self.processor.clone();
        task::spawn(async move {
            let result = processor.query_to_csv(query, output_path).await;
            if let Err(e) = result {
                eprintln!("Failed to export query results to CSV: {:?}", e);
            }
        });
    }

    pub fn show_tables(&self) {
        let processor = self.processor.clone();
        task::spawn(async move {
            let result = processor.show_tables().await;
            if let Err(e) = result {
                eprintln!("Failed to show tables: {:?}", e);
            }
        });
    }
}
