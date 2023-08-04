use axum::{routing::get, Router};
use hyper::{Server, http, body};
use std::net::SocketAddr;
use prometheus::{Encoder, TextEncoder, Registry};
use std::sync::Arc;

pub async fn run_metrics_server(addr: SocketAddr, registry: Arc<Registry>) {
    let app = Router::new()
    .route("/", get(root))
    .route("/health", get(|| async { "OK" }))
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

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> http::Response<hyper::Body> {
    let html = r#"
    <!DOCTYPE html>
    <html>
        <head>
            <title>Library</title>
            <script src="https://unpkg.com/htmx.org@1.9.2"></script>
        </head>
        <body>
            <h1 >Welcome to the SK Collectors Companion</h1>
            <p id="data">Click the button below to get the metrics</p>
            <br />
            <button id="submit" hx-get="/metrics" hx-target='#data' hx-swap="innerHTML">Get Data</button>
        </body>
        <style>
            body {
                background-color: #3d3d3d;
            }
            h1 {
                color: white;
            }
            #data {
                color: white;
            }
            #submit {
                background-color: #4CAF50;
                border: none;
                color: white;
                padding: 15px 32px;
                text-align: center;
                text-decoration: none;
                display: inline-block;
                font-size: 16px;
            }
        </style>
    </html>
    "#;

    http::Response::builder()
        .status(http::StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "text/html")
        .body(hyper::Body::from(html))
        .unwrap()
}
