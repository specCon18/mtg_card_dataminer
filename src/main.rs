use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use tokio::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
struct Card {
    name: String,
    prices: HashMap<String, Option<String>>,
}

#[derive(Debug, Deserialize)]
struct CardFromFile {
    name: String,
    count: usize,
}

#[derive(Debug, Deserialize)]
struct CardFile {
    cards: Vec<CardFromFile>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let file = File::open("cards.json").unwrap();
    let reader = BufReader::new(file);
    let cards_data: CardFile = serde_json::from_reader(reader).unwrap();

    let mut total_card_value_usd = 0.0;

    for card_from_file in &cards_data.cards {
        println!("Card: {}, Count: {}", card_from_file.name, card_from_file.count);

        let request_url = format!(
            "https://api.scryfall.com/cards/named?exact={}",
            card_from_file.name
        );
        let response = reqwest::get(&request_url).await?;
        let card: Card = response.json().await?;
        match card.prices.get("usd") {
            Some(price) => match price {
                Some(price_str) => {
                    match price_str.parse::<f64>() {
                        Ok(price) => {
                            total_card_value_usd += price * card_from_file.count as f64;
                            println!(
                                "Card Name: {}, Price in USD: {}, Count: {}",
                                card.name, price_str, card_from_file.count
                            );
                        }
                        Err(_) => {
                            println!(
                                "Card Name: {}, Price in USD is not a valid number: {}, Count: {}",
                                card.name, price_str, card_from_file.count
                            );
                        }
                    }
                }
                None => println!(
                    "Card Name: {}, No price available in USD, Count: {}",
                    card.name, card_from_file.count
                ),
            },
            None => println!(
                "Card Name: {}, No price information available, Count: {}",
                card.name, card_from_file.count
            ),
        }

        tokio::time::sleep(Duration::from_millis(100)).await; // Adding a delay of 100ms
    }

    println!("Total value of all cards in USD: {}", total_card_value_usd);
    
    Ok(())
}
