use dotenv;
use tokio::time::Duration;
use prometheus::Registry;
use tracing::info;
use std::sync::Arc;

mod server;
mod cards;
mod templates;
mod handlers;
mod util;

use server::run_server;
use cards::{CARD_VALUES,get_data_update_interval};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let update_interval = get_data_update_interval()?;
    let registry = setup_registry()?;
    util::setup_tracing_subscriber();

    let ip_addr = server::get_ip_address()?;
    tokio::spawn(run_server(ip_addr, registry.clone()));

    info!("setting update interval {}", (update_interval/3600).to_string());
    let mut interval = tokio::time::interval(Duration::from_secs(update_interval));

    info!("server started, now listening on port {}", ip_addr.port());
    loop {
        interval.tick().await;
        cards::process_cards(&mut interval).await?;
    }
}

fn setup_registry() -> Result<Arc<Registry>, Box<dyn std::error::Error>> {
    let registry = Arc::new(Registry::new());
    registry.register(Box::new(CARD_VALUES.clone())).unwrap();
    Ok(registry)
}
