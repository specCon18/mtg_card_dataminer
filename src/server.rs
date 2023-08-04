use axum::{routing::get, Router,response::{Html, IntoResponse, Response},http::StatusCode,};
use hyper::{Server, http, body};
use std::net::SocketAddr;
use prometheus::{Encoder, TextEncoder, Registry};
use std::sync::Arc;
use tower_http::services::ServeDir;
use askama::Template;

pub async fn run_metrics_server(addr: SocketAddr, registry: Arc<Registry>) {
    let assets_path = std::env::current_dir().unwrap();
    let app = Router::new()
    .route("/", get(root))
    .nest_service("/assets",ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())))
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

async fn root() -> impl IntoResponse {
    let template = RootTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "root.html")]
struct RootTemplate;

/// A wrapper type that we'll use to encapsulate HTML parsed by askama into valid HTML for axum to serve.
struct HtmlTemplate<T>(T);

    /// Allows us to convert Askama HTML templates into valid HTML for axum to serve in the response.
impl<T> IntoResponse for HtmlTemplate<T>
    where
        T: Template,
    {
        fn into_response(self) -> Response {
            // Attempt to render the template with askama
            match self.0.render() {
                // If we're able to successfully parse and aggregate the template, serve it
                Ok(html) => Html(html).into_response(),
                // If we're not, return an error or some bit of fallback HTML
                Err(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to render template. Error: {}", err),
                )
                    .into_response(),
            }
        }
    }