use sqlx::PgPool;

use crate::{pg, settings};

// Creates a database connection pool based on the settings
// Also wiped the database, so each test can start with a clean db
pub async fn clean_sqlx_pool() -> PgPool {
    let pool = pg::create_pool(&settings::build().database_url).await;

    // Should have a clearing function for each database table:
    clear_users(&pool).await;

    pool
}

// Deletes all users
pub async fn clear_users(pool: &PgPool) {
    sqlx::query!("DELETE FROM users;")
        .execute(pool)
        .await
        .unwrap();
}
