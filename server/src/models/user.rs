use anyhow::Error;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use tracing::{info, instrument, warn};
use uuid::Uuid;

// User for only authentication and authorization.
// Ties together all other data.
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct User {
    // Primary key, not really shown to the user
    pub id: Uuid,
    // Static once user is created
    pub created: DateTime<Utc>,
    // Anytime username, password or something else is updated
    pub changed: DateTime<Utc>,
    // Unique display username
    pub username: String,
    password_hash: String,
}

impl User {
    // Creates a new user.
    #[instrument]
    pub async fn new(pg_pool: &PgPool, username: String, password: String) -> Result<User, anyhow::Error> {
        info!("Requested to create a new user '{}'", username);

        // Check that the username is available
        let username_exists = sqlx::query!("SELECT * FROM users WHERE username = $1", username)
            .fetch_optional(pg_pool)
            .await?;

        if !username_exists.is_none() {
            return Err(anyhow::Error::msg("Username already exists."));
        }

        let hashed_password = tokio::task::spawn_blocking(|| hash_password(password)).await??;

        Ok(sqlx::query_as!(
            User,
            "INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING *;",
            username,
            hashed_password
        )
        .fetch_one(pg_pool)
        .await?)
    }
}

// Generate a hash for a given password.
// Mainly for creating new user accounts or to change a password.
#[instrument]
fn hash_password(password: String) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);

    let hash = Argon2::default().hash_password(&password.as_bytes(), &salt);

    // Some tricks to convert the error, because it doesn't impliment StdError trait
    match hash {
        Ok(hash) => Ok(hash.to_string()),
        Err(error) => {
            warn!("Failed to hash a new password: {}", error);
            Err(Error::msg(
                "Failed to hash password. More information in the server logs.",
            ))
        }
    }
}

// Validates that a pass word matches the one in the gives hash.
// Used validating logins, true is a match false is not a match.
#[instrument]
fn validate_password(password: String, password_hash: String) -> anyhow::Result<bool> {
    let parsed_hash_result = PasswordHash::new(&password_hash);

    // Same conversion as in `fn hash_password`
    let parsed_hash = match parsed_hash_result {
        Ok(value) => value,
        Err(error) => {
            warn!("Failed to validate a password: {}", error);
            return Err(Error::msg("Server failed to validate the password."));
        }
    };

    if Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
    {
        // Correct password
        Ok(true)
    } else {
        // Wrong password
        Ok(false)
    }
}
