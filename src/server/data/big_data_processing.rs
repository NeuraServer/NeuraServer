use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::sync::{Arc, Mutex};
use spark_rs::SparkContext;
use spark_rs::rdd::RDD;

struct AppState {
    spark_context: SparkContext,
}

async fn process_big_data(data: web::Data<Arc<AppState>>) -> impl Responder {
    let sc = &data.spark_context;
    let rdd = sc.text_file("hdfs://path/to/your/bigdata.txt");
    let word_counts = rdd.flat_map(|line| line.split_whitespace())
                         .map(|word| (word, 1))
                         .reduce_by_key(|a, b| a + b);

    let result = word_counts.collect();
    
    HttpResponse::Ok().json(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conf = spark_rs::SparkConf::new().set_app_name("Big Data Processing").set_master("local");
    let spark_context = SparkContext::new(conf);
    
    let data = web::Data::new(Arc::new(AppState { spark_context }));

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(web::resource("/process_big_data").to(process_big_data))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
