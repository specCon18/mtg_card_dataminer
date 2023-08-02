use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest;
use prometheus::{GaugeVec, Opts};

lazy_static::lazy_static! {
    pub static ref CARD_VALUES: GaugeVec = GaugeVec::new(
        Opts::new("card_value", "The value of cards"),
        &["name"]
    ).unwrap();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    name: String,
    prices: HashMap<String, Option<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardFromFile {
    name: String,
    count: usize,
    usd_value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CardFile {
    pub cards: Vec<CardFromFile>,
}

pub async fn process_card(card_from_file: &mut CardFromFile) -> Result<(), Box<dyn std::error::Error>> {
    let request_url = format!(
        "https://api.scryfall.com/cards/named?exact={}",
        card_from_file.name
    );
    let response = reqwest::get(&request_url).await?;
    let card: Card = response.json().await?;

    if let Some(price) = card.prices.get("usd") {
        if let Some(price_str) = price {
            card_from_file.usd_value = Some(price_str.clone());

            let value = price_str.parse::<f64>().unwrap();
            CARD_VALUES.with_label_values(&[&card_from_file.name]).set(value);
        } else {
            card_from_file.usd_value = Some("0.0".to_string());
            CARD_VALUES.with_label_values(&[&card_from_file.name]).set(0.0);
        }
    }

    Ok(())
}

pub fn process_top_cards(cards: &mut Vec<CardFromFile>) {
    cards.sort_by(|a, b| b.usd_value.partial_cmp(&a.usd_value).unwrap());
    let top_10_cards = &cards[..10];
    for card in top_10_cards {
        if let Some(price_str) = &card.usd_value {
            let value = price_str.parse::<f64>().unwrap();
            CARD_VALUES.with_label_values(&[&card.name]).set(value);
        }
    }
}