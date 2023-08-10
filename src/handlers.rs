use axum::response::IntoResponse;
use hyper::StatusCode;

use crate::cards::{read_card_file, CardForTemplate};
use crate::templates::RootTemplate;
use crate::util::HtmlTemplate;

pub async fn root() -> Result<HtmlTemplate<RootTemplate>, (StatusCode, String)> {
    let file_path = "test_data/test.json";
    match read_card_file(file_path).await {
        Ok(card_file) => {
            let cards_for_template: Vec<_> = card_file.cards
                .into_iter()
                .map(|card| CardForTemplate {
                    name: card.name,
                    count: card.count,
                    usd_value: card.usd_value.unwrap_or_else(|| "N/A".into()),
                })
                .collect();
            let template = RootTemplate {
                name: "Steven".to_string(),
                cards: cards_for_template,
            };
            Ok(HtmlTemplate(template))
        },
        Err(err) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("An error occurred: {}", err)))
        },
    }
}

pub async fn health() -> impl IntoResponse {
    "200 OK"
}