mod ping;

use axum::{routing::get, Router};
use sqlx::PgPool;
use tracing::{info, instrument};

// Centralized builder for all API routes.
// Actual routes are also under this module,
// and not public the rest of the crate.
#[instrument]
pub fn build_router(pg_pool: &PgPool) -> Router {
    info!("Building axum routes");

    Router::new().route("/ping", get(ping::handle))
}
