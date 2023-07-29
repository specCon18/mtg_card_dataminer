use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs::File, fs::OpenOptions, io::BufReader};
use tokio::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};


/// A struct to represent a Card returned from the API.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Card {
    /// The name of the card.
    name: String,
    /// The prices of the card in various formats.
    prices: HashMap<String, Option<String>>,
}

/// A struct to represent a Card from the local JSON file.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct CardFromFile {
    /// The name of the card.
    name: String,
    /// The count of this card.
    count: usize,
    /// The value of this card in USD.
    usd_value: Option<String>,
}

/// A struct to represent a collection of Cards from the local JSON file.
#[derive(Serialize, Deserialize, Debug)]
struct CardFile {
    /// The list of cards.
    cards: Vec<CardFromFile>,
}

/// The main function.
///
/// This function reads a local JSON file of cards, sends an API request for each card to get the current price in USD,
/// compares the fetched price with the stored price in the local file, and updates the file if there is any difference.
///
/// Note: There is a delay of 100ms between each API request as per the API rules.
#[tokio::main]
async fn main() -> Result<(), Box<std::io::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide the path to the JSON file as an argument.");
        return Ok(());
    }

    let file_path = &args[1];
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("There was a problem opening the file: {:?}", error);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File not found",
            )));
        }
    };    
    
    let reader = BufReader::new(file);
    let mut cards_data: CardFile = serde_json::from_reader(reader).unwrap();

    let pb = ProgressBar::new(cards_data.cards.len() as u64);
    let style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap();
    let style = style.progress_chars("#>-");
    pb.set_style(style);

    for card_from_file in &mut cards_data.cards {
        let request_url = format!(
            "https://api.scryfall.com/cards/named?exact={}",
            card_from_file.name
        );
        let response = reqwest::get(&request_url).await
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.to_string()))?;
    
        let card: Card = response.json().await
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.to_string()))?;

        

        if let Some(price) = card.prices.get("usd") {
            if let Some(price_str) = price {
                if card_from_file.usd_value.as_ref() != Some(price_str) {
                    card_from_file.usd_value = Some(price_str.clone());
                }
            }
        }

        pb.inc(1);
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    pb.finish_with_message("Completed!");

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)
        .unwrap();

    serde_json::to_writer_pretty(file, &cards_data).unwrap();

    Ok(())
}

