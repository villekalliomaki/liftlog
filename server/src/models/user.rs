use std::fmt::{Debug, Display};

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
    RequestPartsExt,
};
use axum_extra::{TypedHeader, headers::{Authorization, authorization::Bearer}};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{debug, error, info, instrument};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{api::response::RouteError, models::access_token::AccessToken};

// User for authentication and authorization
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema)]
pub struct User {
    // Primary key, not really shown to the user
    pub id: Uuid,
    // Static once user is created
    pub created: DateTime<Utc>,
    // Changes to current time anytime something else is updated
    pub changed: DateTime<Utc>,
    // Unique display username
    pub username: String,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub password_hash: String,
}

impl User {
    // Check that the username is available and if so, create a new user
    #[instrument(skip(password))]
    pub async fn new(
        username: impl ToString + Debug + Display,
        password: impl ToString + Debug,
        pool: &PgPool,
    ) -> Result<User, RouteError> {
        info!("Trying to create a new user '{}'", username);

        let username_string = username.to_string();

        // Check if username is available
        is_username_available(&username_string, pool).await?;

        // Hash the user's password
        let hashed_password = hash_password(password.to_string())?;

        // Create the new user
        let new_user = sqlx::query_as!(
            User,
            "INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING *",
            username_string,
            hashed_password
        )
        .fetch_one(pool)
        .await?;

        info!("New user '{}' created", new_user.username);

        Ok(new_user)
    }

    #[instrument]
    pub async fn change_username(
        &mut self,
        new_username: impl ToString + Debug + Display,
        pool: &PgPool,
    ) -> Result<(), RouteError> {
        info!(
            "Trying to change username {} to {}",
            self.username, new_username
        );

        let new_username_string = new_username.to_string();

        // Check availability
        is_username_available(&new_username_string, pool).await?;

        // If available, change the usename
        let updated = sqlx::query!(
            "UPDATE users SET username = $1, changed = NOW() WHERE id = $2 RETURNING username, changed",
            new_username_string,
            self.id
        )
        .fetch_one(pool)
        .await?;

        self.username = updated.username;
        self.changed = updated.changed;

        Ok(())
    }

    #[instrument(skip(password))]
    pub async fn from_credentials(
        username: impl ToString + Debug + Display,
        password: impl ToString + Debug,
        pool: &PgPool,
    ) -> Result<User, RouteError> {
        info!("Trying to get user {} from credentials", username);

        // Find the user by the username
        let potential_user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE username = $1",
            username.to_string()
        )
        .fetch_optional(pool)
        .await?;

        let user = match potential_user {
            Some(user) => user,
            None => {
                return Err(RouteError::new(
                    "Username not found.",
                    Some("username"),
                    StatusCode::NOT_FOUND,
                ))
            }
        };

        // Validate that the password matches
        validate_password_hash(password.to_string(), &user.password_hash)?;

        Ok(user)
    }

    #[instrument(skip(new_password))]
    pub async fn change_password(
        &mut self,
        new_password: impl ToString,
        pool: &PgPool,
    ) -> Result<(), RouteError> {
        let new_hashed_password = hash_password(new_password.to_string())?;

        let updates = sqlx::query!(
            "UPDATE users SET password_hash = $1, changed = NOW() WHERE id = $2 RETURNING changed, password_hash",
            new_hashed_password,
            self.id
        )
        .fetch_one(pool)
        .await?;

        self.password_hash = updates.password_hash;
        self.changed = updates.changed;

        Ok(())
    }

    #[instrument]
    pub async fn delete(self, pool: &PgPool) -> Result<(), RouteError> {
        sqlx::query!("DELETE FROM users WHERE id = $1", self.id)
            .execute(pool)
            .await?;

        Ok(())
    }
}

// Derive an user from a request containing a valid access token in the headers
// From https://docs.rs/axum/latest/axum/extract/struct.State.html#for-library-authors
#[async_trait]
impl<S> FromRequestParts<S> for User
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = RouteError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        debug!("Trying to get an user from the 'Authorization' header");

        let TypedHeader(Authorization(bearer_token)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| {
                RouteError::new(
                    "Authorization header is missing.",
                    None::<&str>,
                    StatusCode::BAD_REQUEST,
                )
            })?;

        let pool = &PgPool::from_ref(state);

        let access_token = AccessToken::from_token(bearer_token.token(), pool).await?;

        Ok(access_token.get_user(pool).await?)
    }
}

// Returns an error if username is taken
#[instrument]
async fn is_username_available(username: &String, pool: &PgPool) -> Result<(), RouteError> {
    let username_count = sqlx::query_scalar!(
        "SELECT COUNT(*) AS count FROM users WHERE username = $1",
        username
    )
    .fetch_one(pool)
    .await?;

    if username_count.unwrap_or(1) != 0 {
        info!("Username {} was unavailable", username);

        return Err(RouteError::new(
            "Username is already taken.",
            Some("username"),
            StatusCode::CONFLICT,
        ));
    }

    Ok(())
}

// Hash a password
#[instrument(skip(password))]
fn hash_password(password: String) -> Result<String, RouteError> {
    let salt = SaltString::generate(&mut OsRng).to_owned();

    let hash_result = Argon2::default().hash_password(&password.as_bytes(), &salt);

    return match hash_result {
        Ok(hash) => Ok(hash.to_string()),
        Err(error) => {
            error!("Failed to hash a password: {}", error);
            Err(RouteError::new(
                "Failed to hash the password.",
                Some("password"),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    };
}

// Validate that the given password matched the given hash
#[instrument(skip(password))]
fn validate_password_hash(password: String, hash: &String) -> Result<(), RouteError> {
    let parsed_hash_result = PasswordHash::new(hash);

    let parsed_hash = match parsed_hash_result {
        Ok(parsed) => parsed,
        Err(error) => {
            error!("Failed to parse a password hash: {}", error);
            return Err(RouteError::new(
                "Failed to parse the user's password hash.",
                None::<&str>,
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    if Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
    {
        Ok(())
    } else {
        Err(RouteError::new(
            "Wrong password.",
            Some("password"),
            StatusCode::UNAUTHORIZED,
        ))
    }
}

// User tests interact with the database, so there are helper functions to prepare it
#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::response::RouteError;

    const PASSWORD: &str = "3pWi7ttGSVVWLj";
    const USERNAME: &str = "test_username";

    #[sqlx::test]
    async fn username_conflict(pool: PgPool) {
        let user1: Result<User, RouteError> = User::new(USERNAME, PASSWORD, &pool).await;

        // First user should be fine
        assert!(user1.is_ok());

        let user2: Result<User, RouteError> = User::new(USERNAME, PASSWORD, &pool).await;

        // Should not have been created
        assert!(user2.is_err());
    }

    #[sqlx::test]
    async fn changed_column_update(pool: PgPool) {
        let mut user: User = User::new(USERNAME, PASSWORD, &pool).await.unwrap();
        let initial_changed_column = user.changed.clone();

        // Change username
        user.change_username("new_test_username", &pool)
            .await
            .unwrap();

        // Field should have changed
        assert_ne!(initial_changed_column, user.changed);

        // Also database should have updated
        let user_from_db: User = User::from_credentials("new_test_username", PASSWORD, &pool)
            .await
            .unwrap();

        assert_ne!(initial_changed_column, user_from_db.changed);
    }

    #[sqlx::test]
    async fn valid_password(pool: PgPool) {
        User::new(USERNAME, PASSWORD, &pool).await.unwrap();

        // Get user from database with the same password
        assert!(User::from_credentials(USERNAME, PASSWORD, &pool)
            .await
            .is_ok());
    }

    #[sqlx::test]
    async fn invalid_password(pool: PgPool) {
        User::new(USERNAME, PASSWORD, &pool).await.unwrap();

        // Get user from database with a wrong password
        assert!(
            User::from_credentials(USERNAME, "some_other_password", &pool)
                .await
                .is_err()
        );
    }

    #[sqlx::test]
    async fn password_change(pool: PgPool) {
        let mut user: User = User::new(USERNAME, PASSWORD, &&pool).await.unwrap();

        let new_password = "some_other_password_2";

        // Change the password of the user
        assert!(user.change_password(new_password, &&pool).await.is_ok());

        // Try the old password, should not work
        assert!(User::from_credentials(USERNAME, PASSWORD, &&pool)
            .await
            .is_err());

        // Try new pasword, should work
        assert!(User::from_credentials(USERNAME, new_password, &&pool)
            .await
            .is_ok());
    }

    #[sqlx::test]
    async fn delete(pool: PgPool) {
        let user: User = User::new(USERNAME, PASSWORD, &pool).await.unwrap();

        // Delete the user
        user.delete(&pool).await.unwrap();

        // Make sure it was deleted
        assert!(User::from_credentials(USERNAME, PASSWORD, &pool)
            .await
            .is_err());
    }
}
