use std::{process::exit, time::Duration};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing::{error, info, instrument};

// Connect to the database and create the database pool,
// and tests that the connection works.
// Exits the program on failure.
#[instrument]
pub async fn create_pool(url: &str) -> Pool<Postgres> {
    info!("Creating PostgreSQL database client");

    let pool_result = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(url)
        .await;

    let pool = match pool_result {
        Ok(pool) => pool,
        Err(error) => {
            error!("Failed to create a PostgreSQL connection pool: {}", error);
            exit(1);
        }
    };

    info!("Making a test query with the PostgreSQL connection");

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let query_result: Result<(i64,), sqlx::Error> = sqlx::query_as("SELECT $1")
        .bind(1_i64)
        .fetch_one(&pool)
        .await;

    if query_result.is_err() {
        error!(
            "Failed to execute PostgreSQL database connection test query: {:?}",
            query_result
        );
        exit(1);
    }

    info!("Running migrations");
    if let Err(error) = sqlx::migrate!().run(&pool).await {
        error!("Failed to apply migratios: {}", error);
        exit(1);
    }

    info!("PostgreSQL connection pool created");

    pool
}
