use axum::{routing::get, Router};
use std::{
    net::{AddrParseError, SocketAddr},
    process::exit,
};
use tracing::{error, info};

// Initialize and start HTTP server
pub async fn start(addr: &str) {
    info!("Starting HTTP server on {}", addr);

    // Parse listen address
    let parsed_addr_result: Result<SocketAddr, AddrParseError> = addr.parse();

    let parsed_addr = match parsed_addr_result {
        Ok(parsed) => parsed,
        Err(error) => {
            error!(
                "Failed to parse HTTP server listen address '{}': {}",
                addr, error
            );
            exit(1);
        }
    };

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run it with hyper on localhost:3000
    axum::Server::bind(&parsed_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
