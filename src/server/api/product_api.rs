use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Product {
    id: u32,
    name: String,
    price: f64,
}

struct AppState {
    products: Mutex<Vec<Product>>,
}

async fn get_products(data: web::Data<AppState>) -> impl Responder {
    let products = data.products.lock().unwrap();
    HttpResponse::Ok().json(&*products)
}

async fn get_product(data: web::Data<AppState>, product_id: web::Path<u32>) -> impl Responder {
    let products = data.products.lock().unwrap();
    if let Some(product) = products.iter().find(|p| p.id == *product_id) {
        HttpResponse::Ok().json(product)
    } else {
        HttpResponse::NotFound().body("Product not found")
    }
}

async fn create_product(data: web::Data<AppState>, product: web::Json<Product>) -> impl Responder {
    let mut products = data.products.lock().unwrap();
    products.push(product.into_inner());
    HttpResponse::Created().body("Product created")
}

async fn delete_product(data: web::Data<AppState>, product_id: web::Path<u32>) -> impl Responder {
    let mut products = data.products.lock().unwrap();
    if let Some(pos) = products.iter().position(|p| p.id == *product_id) {
        products.remove(pos);
        HttpResponse::Ok().body("Product deleted")
    } else {
        HttpResponse::NotFound().body("Product not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let products = web::Data::new(AppState {
        products: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(products.clone())
            .route("/products", web::get().to(get_products))
            .route("/products/{id}", web::get().to(get_product))
            .route("/products", web::post().to(create_product))
            .route("/products/{id}", web::delete().to(delete_product))
    })
    .bind("127.0.0.1:5500")?
    .run()
    .await
}
