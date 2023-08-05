use axum::response::IntoResponse;

use crate::templates::RootTemplate;
use crate::util::HtmlTemplate;

pub async fn root() -> impl IntoResponse {
    let template = RootTemplate {};
    HtmlTemplate(template)
}

pub async fn health() -> impl IntoResponse {
    "200 OK"
}