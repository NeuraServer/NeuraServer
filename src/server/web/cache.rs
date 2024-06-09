use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use actix_web::{web, HttpResponse, Responder};

struct Cache {
    store: Mutex<HashMap<String, String>>,
}

async fn set_item(data: web::Data<Arc<Cache>>, key: web::Path<String>, value: web::Json<String>) -> impl Responder {
    let mut store = data.store.lock().unwrap();
    store.insert(key.into_inner(), value.into_inner());
    HttpResponse::Ok().body("Item set")
}

async fn get_item(data: web::Data<Arc<Cache>>, key: web::Path<String>) -> impl Responder {
    let store = data.store.lock().unwrap();
    if let Some(value) = store.get(&key.into_inner()) {
        HttpResponse::Ok().body(value.clone())
    } else {
        HttpResponse::NotFound().body("Item not found")
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    let cache = web::Data::new(Arc::new(Cache {
        store: Mutex::new(HashMap::new()),
    }));
    cfg.app_data(cache.clone())
        .service(web::resource("/cache/{key}").route(web::put().to(set_item)))
        .service(web::resource("/cache/{key}").route(web::get().to(get_item)));
}
