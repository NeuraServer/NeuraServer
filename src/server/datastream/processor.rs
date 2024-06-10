use crate::datafusion::DataFusionContext;
use std::sync::{Arc, Mutex};
use datafusion::error::Result;
use datafusion::arrow::record_batch::RecordBatch;

pub struct DataProcessor {
    context: Arc<Mutex<DataFusionContext>>,
}

impl DataProcessor {
    pub fn new() -> Self {
        let context = Arc::new(Mutex::new(DataFusionContext::new()));
        Self { context }
    }

    pub fn register_csv(&self, name: &str, path: &str) -> Result<()> {
        let mut ctx = self.context.lock().unwrap();
        ctx.register_csv(name, path)
    }

    pub fn register_parquet(&self, name: &str, path: &str) -> Result<()> {
        let mut ctx = self.context.lock().unwrap();
        ctx.register_parquet(name, path)
    }

    pub async fn execute_query(&self, query: &str) -> Result<Vec<RecordBatch>> {
        let mut ctx = self.context.lock().unwrap();
        ctx.execute_query(query).await
    }

    pub async fn query_to_csv(&self, query: &str, output_path: &str) -> Result<()> {
        let mut ctx = self.context.lock().unwrap();
        ctx.query_to_csv(query, output_path).await
    }

    pub async fn show_tables(&self) -> Result<()> {
        let mut ctx = self.context.lock().unwrap();
        ctx.show_tables().await
    }
}
