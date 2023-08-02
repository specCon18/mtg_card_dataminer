use axum::{routing::get, Router};
use hyper::{Server, http, body};
use std::net::SocketAddr;
use prometheus::{Encoder, TextEncoder, Registry};
use std::sync::Arc;

pub async fn run_metrics_server(addr: SocketAddr, registry: Arc<Registry>) {
    let app = Router::new().route("/metrics", get(move || {
        let registry = Arc::clone(&registry);
        async move {
            let metric_families = registry.gather();
            let mut buffer = vec![];
            let encoder = TextEncoder::new();
            encoder.encode(&metric_families, &mut buffer).unwrap();
            let metrics = String::from_utf8(buffer).unwrap();
            http::Response::new(body::Body::from(metrics))
        }
    }));

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
