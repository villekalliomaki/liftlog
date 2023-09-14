mod api;
mod http_server;
mod models;
mod pg;
mod settings;

use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    init_tracing();

    info!("Starting LiftLog");

    let current_setttings = settings::build();

    let pg_pool = pg::create_pool(&current_setttings.database_url).await;

    http_server::start(&current_setttings.listen_address, pg_pool).await;
}

// Initialize tracing library for logging
fn init_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("Tracing initialized")
}
