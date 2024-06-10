use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::sync::{Arc, Mutex};
use neo4rs::{Graph, query};

struct AppState {
    graph: Graph,
}

async fn query_graph(data: web::Data<Arc<AppState>>) -> impl Responder {
    let graph = &data.graph;
    let result = graph.execute(query("MATCH (n) RETURN n")).await.unwrap();

    let mut nodes = Vec::new();
    for row in result {
        let node: String = row.get("n").unwrap();
        nodes.push(node);
    }

    HttpResponse::Ok().json(nodes)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let graph = Graph::new("localhost:7687", "neo4j", "password").await.unwrap();

    let data = web::Data::new(Arc::new(AppState { graph }));

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(web::resource("/query_graph").to(query_graph))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
