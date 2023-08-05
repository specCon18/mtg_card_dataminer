use axum::{routing::get, Router};
use hyper::{Server, http, body};
use tower_http::services::ServeDir;
use std::net::SocketAddr;
use prometheus::{Encoder, TextEncoder, Registry};
use std::sync::Arc;

use crate::handlers;

pub async fn run_server(addr: SocketAddr, registry: Arc<Registry>) {
    let assets_path = std::env::current_dir().unwrap();
    let app = Router::new()
    .nest_service("/assets",ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())))
    .route("/", get(handlers::root))
    .route("/health", get(handlers::health))
    .route("/metrics", get(move || {
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

    Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

pub fn get_ip_address() -> Result<SocketAddr, Box<dyn std::error::Error>> {
    let ip_addr = SocketAddr::from(([127, 0, 0, 1],
        dotenv::var("PORT")
            .map_err(|_| "PORT is not defined in the .env file")?
            .parse::<u16>()
            .map_err(|_| "PORT is not a valid number")?
    ));

    Ok(ip_addr)
}