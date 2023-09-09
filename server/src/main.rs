mod http_server;
mod settings;

use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    init_tracing();

    info!("Starting LiftLog");

    let current_setttings = settings::build();

    http_server::start(&current_setttings.listen_address).await;
}

// Initialize tracing library for logging
fn init_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("Tracing initialized")
}
