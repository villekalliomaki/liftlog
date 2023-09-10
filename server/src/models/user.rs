use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

// User for only authentication and authorization.
// Ties together all other data.
#[derive(Serialize, FromRow)]
pub struct User {
    // Primary key, not really shown to the user
    id: Uuid,
    // Static once user is created
    created: DateTime<Utc>,
    // Anytime username, password or something else is updated
    changed: DateTime<Utc>,
    // Unique display username
    username: String,
    password_hash: String,
}

impl User {
    // Creates a new user.
    pub async fn new(pg_pool: &PgPool, username: String, password: String) {}
}

fn hash_password() {}

fn validate_password() {}
