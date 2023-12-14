#![forbid(unsafe_code)]
#![allow(dead_code)]

mod api;
mod http_server;
mod models;
mod pg;
mod settings;
mod test_utils;

use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    init_tracing();

    info!("Starting LiftLog ...");

    let current_setttings = settings::build();

    let pool = pg::create_pool(&current_setttings.database_url).await;

    http_server::start(&current_setttings.listen_address, pool).await;
}

// Initialize tracing library for logging
fn init_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("Tracing initialized")
}
