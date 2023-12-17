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
    let current_settings = settings::build();

    init_tracing(current_settings.debug);

    info!("Starting LiftLog ...");

    let pool = pg::create_pool(&current_settings.database_url).await;

    http_server::start(&current_settings.listen_address, pool).await;
}

// Initialize tracing library for logging
fn init_tracing(debug: bool) {
    let level = match debug {
        true => Level::DEBUG,
        false => Level::INFO,
    };

    let subscriber = FmtSubscriber::builder().with_max_level(level).finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("Tracing initialized")
}
