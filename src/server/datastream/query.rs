use datafusion::prelude::*;
use datafusion::arrow::record_batch::RecordBatch;
use datafusion::error::Result;
use std::sync::Arc;

pub struct DataFusionContext {
    ctx: ExecutionContext,
}

impl DataFusionContext {
    pub fn new() -> Self {
        let mut ctx = ExecutionContext::new();
        Self { ctx }
    }

    pub fn register_csv(&mut self, name: &str, path: &str) -> Result<()> {
        self.ctx.register_csv(name, path, CsvReadOptions::new())
    }

    pub async fn execute_query(&mut self, query: &str) -> Result<Vec<RecordBatch>> {
        let df = self.ctx.sql(query)?;
        df.collect().await
    }

    pub fn register_parquet(&mut self, name: &str, path: &str) -> Result<()> {
        self.ctx.register_parquet(name, path)
    }

    pub async fn show_tables(&mut self) -> Result<()> {
        let tables = self.ctx.tables();
        for table in tables {
            println!("{}", table);
        }
        Ok(())
    }

    pub async fn query_to_csv(&mut self, query: &str, output_path: &str) -> Result<()> {
        let df = self.ctx.sql(query)?;
        let batches = df.collect().await?;
        let mut file = std::fs::File::create(output_path)?;
        let mut writer = datafusion::arrow::csv::Writer::new(&mut file);
        for batch in batches {
            writer.write(&batch)?;
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut ctx = DataFusionContext::new();
    ctx.register_csv("example", "path/to/example.csv")?;
    let batches = ctx.execute_query("SELECT * FROM example").await?;

    for batch in batches {
        println!("{:?}", batch);
    }

    Ok(())
}
