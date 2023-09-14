use sqlx::PgPool;
use std::{
    net::{AddrParseError, SocketAddr},
    process::exit,
};
use tracing::{error, info, instrument};

use crate::api::routes;

// Initialize and start HTTP server
#[instrument]
pub async fn start(addr: &str, pg_pool: PgPool) {
    info!("Starting HTTP server on http://{}", addr);

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
    let app_router = routes::build_router(pg_pool);

    // run it with hyper on localhost:3000
    axum::Server::bind(&parsed_addr)
        .serve(app_router.into_make_service())
        .await
        .unwrap();
}
