use dotenv;
use std::{env, fs::File, fs::OpenOptions, io::BufReader, sync::Arc,net::SocketAddr};
use tokio::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};
use prometheus::Registry;

mod server;
mod card;

use server::run_metrics_server;
use card::{CARD_VALUES, CardFile, process_card, process_top_cards};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    
    let update_interval = dotenv::var("UPDATE_INTERVAL")
        .map_err(|_| "UPDATE_INTERVAL is not defined in the .env file")?
        .parse::<u64>()
        .map_err(|_| "UPDATE_INTERVAL is not a valid number")?
        * 3600;

    let registry = Arc::new(Registry::new());
    registry.register(Box::new(CARD_VALUES.clone())).unwrap();

    let metrics_addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    tokio::spawn(run_metrics_server(metrics_addr, registry.clone()));

    let mut interval = tokio::time::interval(Duration::from_secs(update_interval));
    loop {
        interval.tick().await;

        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            eprintln!("Please provide the path to the JSON file as an argument.");
            continue;
        }

        let file_path = &args[1];
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut cards_data: CardFile = serde_json::from_reader(reader)?;

        let pb = setup_progress_bar(cards_data.cards.len() as u64);
        for card_from_file in &mut cards_data.cards {
            process_card(card_from_file).await?;
            pb.inc(1);
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        process_top_cards(&mut cards_data.cards);
        pb.finish_with_message("Completed!");

        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path)?;

        serde_json::to_writer_pretty(file, &cards_data)?;
    }
}

fn setup_progress_bar(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);
    let style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-");
    pb.set_style(style);
    pb
}