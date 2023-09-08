use axum::{routing::get, Router};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use crate::config::Config;


#[tokio::main]
async fn main() {
    init_tracing();

    let config = Config::from_env();

    init_server().await;
}

/**
 * Initialize tracing library for logging
 */
fn init_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();
}

/**
 * Initialize and start HTTP server
 */
async fn init_server() {
    info!("Starting server");

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

