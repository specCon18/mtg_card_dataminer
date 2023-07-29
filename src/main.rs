/// This program utilizes several external libraries to perform its functions, such as dotenv, serde, tokio, etc.
use dotenv;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs::File, fs::OpenOptions, io::BufReader};
use tokio::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};

/// This structure defines a Card object, consisting of a name and a hashmap of prices.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Card {
    name: String,
    prices: HashMap<String, Option<String>>,
}

/// This structure defines a CardFromFile object, consisting of a name, count, and a USD value.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct CardFromFile {
    name: String,
    count: usize,
    usd_value: Option<String>,
}

/// This structure defines a CardFile object, consisting of a vector of CardFromFile objects.
#[derive(Serialize, Deserialize, Debug)]
struct CardFile {
    cards: Vec<CardFromFile>,
}

/// The main function of the program. It retrieves card price information from an external API and updates a local JSON file.
/// It accepts a path to a JSON file as an argument and also uses an environment variable, UPDATE_INTERVAL, to determine the frequency of its update cycle.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    // Retrieve the update interval from the .env file or return an error if not present.
    let update_interval_str = match dotenv::var("UPDATE_INTERVAL") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("UPDATE_INTERVAL is not defined in the .env file");
            return Err("UPDATE_INTERVAL is not defined in the .env file".into());
        }
    };
    // Attempt to parse the update interval as a u64 or return an error if it fails.
    let update_interval = match update_interval_str.parse::<u64>() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("UPDATE_INTERVAL is not a valid number");
            return Err("UPDATE_INTERVAL is not a valid number".into());
        }
    };

    let update_interval = std::time::Duration::from_secs(update_interval * 3600);

    let mut interval = tokio::time::interval(update_interval);
    loop {
        interval.tick().await;

        // Retrieve the file path from the program's arguments or return an error if not present.
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            eprintln!("Please provide the path to the JSON file as an argument.");
            return Ok(());
        }

        let file_path = &args[1];
        let file = File::open(file_path)?;

        let reader = BufReader::new(file);
        let mut cards_data: CardFile = serde_json::from_reader(reader)?;

        // Setting up a progress bar for visual representation of the card processing progress.
        let pb = ProgressBar::new(cards_data.cards.len() as u64);
        let style = ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap();
        let style = style.progress_chars("#>-");
        pb.set_style(style);

        // For each card in the input file, retrieve the current price information and update the local data if necessary.
        for card_from_file in &mut cards_data.cards {
            let request_url = format!(
                "https://api.scryfall.com/cards/named?exact={}",
                card_from_file.name
            );
            let response = reqwest::get(&request_url).await?;

            let card: Card = response.json().await?;

            if let Some(price) = card.prices.get("usd") {
                if let Some(price_str) = price {
                    if card_from_file.usd_value.as_ref() != Some(price_str) {
                        card_from_file.usd_value = Some(price_str.clone());
                    }
                }
            }

            // Increment the progress bar and pause for a brief period.
            pb.inc(1);
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        // Mark the progress bar as completed.
        pb.finish_with_message("Completed!");

        // Write the updated card data back to the input file.
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path)?;

        serde_json::to_writer_pretty(file, &cards_data)?;

    }
}
