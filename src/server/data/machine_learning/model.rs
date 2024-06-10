use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use rand::Rng;
use linregress::LinearRegression;

#[derive(Deserialize)]
struct TrainRequest {
    input: Vec<f64>,
    output: f64,
}

#[derive(Deserialize)]
struct PredictRequest {
    input: Vec<f64>,
}

#[derive(Serialize)]
struct PredictResponse {
    output: f64,
}

struct AppState {
    model: Mutex<LinearRegression<f64>>,
}

async fn train(data: web::Data<Arc<AppState>>, req: web::Json<TrainRequest>) -> impl Responder {
    let mut model = data.model.lock().unwrap();

    model.add_data_point(req.input.clone(), req.output);

    HttpResponse::Ok().body("Model trained with new data point")
}

async fn predict(data: web::Data<Arc<AppState>>, req: web::Json<PredictRequest>) -> impl Responder {
    let model = data.model.lock().unwrap();

    let prediction = model.predict(&req.input).unwrap_or(0.0);
    let response = PredictResponse { output: prediction };

    HttpResponse::Ok().json(response)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let initial_weights: Vec<f64> = (0..3).map(|_| rand::thread_rng().gen_range(-1.0..1.0)).collect();
    let model = LinearRegression::new(initial_weights, 2.0);
    
    let data = web::Data::new(Arc::new(AppState {
        model: Mutex::new(model),
    }));

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(actix_web::middleware::Logger::default())
            .route("/train", web::post().to(train))
            .route("/predict", web::post().to(predict))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
