use prometheus::{Encoder, TextEncoder, Counter, Opts, Registry};
use actix_web::{web, HttpResponse};

pub struct Metrics {
    pub request_count: Counter,
    pub registry: Registry,
}

impl Metrics {
    pub fn new() -> Metrics {
        let request_count_opts = Opts::new("request_count", "Number of requests received");
        let request_count = Counter::with_opts(request_count_opts).unwrap();
        let registry = Registry::new();
        registry.register(Box::new(request_count.clone())).unwrap();

        Metrics {
            request_count,
            registry,
        }
    }

    pub fn increment_request_count(&self) {
        self.request_count.inc();
    }

    pub fn gather_metrics(&self) -> String {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }
}

pub async fn get_metrics(metrics: web::Data<Metrics>) -> HttpResponse {
    let metrics_data = metrics.gather_metrics();
    HttpResponse::Ok().body(metrics_data)
}
