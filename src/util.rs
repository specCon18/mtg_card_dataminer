use axum::{response::{Html,Response},http::StatusCode,response::IntoResponse};
use askama::Template;
use indicatif::{ProgressBar, ProgressStyle};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


/// A wrapper type that we'll use to encapsulate HTML parsed by askama into valid HTML for axum to serve.
pub struct HtmlTemplate<T>(pub T);

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

pub fn setup_progress_bar(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);
    let style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>.");
    pb.set_style(style);
    pb
}

pub fn setup_tracing_subscriber() {
    tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "sk_tcg_trader".into()),)
    .with(tracing_subscriber::fmt::layer())
    .init();
}