use serde::{Deserialize, Serialize};
use reqwest;
use prometheus::{GaugeVec, Opts, Registry};
use std::{env, fs::File, fs::OpenOptions, io::BufReader, collections::HashMap,sync::Arc};
use crate::util;

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
    pub name: String,
    pub count: i32,
    pub usd_value: Option<String>,
}

pub struct CardForTemplate {
    pub name: String,
    pub count: i32,
    pub usd_value: String, // No longer an Option
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

pub fn get_data_update_interval() -> Result<u64, Box<dyn std::error::Error>> {
    let update_interval = dotenv::var("UPDATE_INTERVAL")
        .map_err(|_| "UPDATE_INTERVAL is not defined in the .env file")?
        .parse::<u64>()
        .map_err(|_| "UPDATE_INTERVAL is not a valid number")?;
        
    Ok(update_interval * 3600)
}

pub fn process_export_data(cards: &mut Vec<CardFromFile>) {
    cards.sort_by(|a, b| b.usd_value.partial_cmp(&a.usd_value).unwrap());
    let cards = &cards[..10];
    for card in cards {
        if let Some(price_str) = &card.usd_value {
            let value = price_str.parse::<f64>().unwrap();
            CARD_VALUES.with_label_values(&[&card.name]).set(value);
        }
    }
}

pub async fn process_cards(interval: &mut tokio::time::Interval) -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide the path to the JSON file as an argument.");
        return Ok(());
    }

    let file_path = &args[1];
    let mut cards_data = read_card_file(file_path).await?;
    let pb = util::setup_progress_bar(cards_data.cards.len() as u64);

    for card_from_file in &mut cards_data.cards {
        process_card(card_from_file).await?;
        pb.inc(1);
    }
    interval.tick().await;

    process_export_data(&mut cards_data.cards);
    pb.finish_with_message("Completed!");

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;

    serde_json::to_writer_pretty(file, &cards_data)?;
    Ok(())
}


pub fn setup_registry() -> Result<Arc<Registry>, Box<dyn std::error::Error>> {
    let registry = Arc::new(Registry::new());
    registry.register(Box::new(CARD_VALUES.clone())).unwrap();
    Ok(registry)
}

pub async fn read_card_file(file_path: &str) -> Result<CardFile, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let cards_data: CardFile = serde_json::from_reader(reader)?;
    Ok(cards_data)
}
