use std::fmt::{Debug, Display};

use axum::http::StatusCode;
use chrono::{DateTime, Duration, Utc};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{error, info, instrument, warn};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::api::response::RouteError;

use super::user::User;

// Used to authenticate single API requests to a specific user
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema)]
pub struct AccessToken {
    pub token: String,
    pub created: DateTime<Utc>,
    pub expires: DateTime<Utc>,
    pub user_id: Uuid,
}

impl AccessToken {
    // Creates a new token valid for now+validity for an user
    #[instrument]
    pub async fn new(
        user_id: Uuid,
        validity: Duration,
        pool: &PgPool,
    ) -> Result<AccessToken, RouteError> {
        info!("Creating new access token for {}", user_id);

        let token: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(50)
            .map(char::from)
            .collect();

        let expiration_time = Utc::now() + validity;

        Ok(sqlx::query_as!(
            AccessToken,
            "INSERT INTO access_tokens (token, expires, user_id) VALUES ($1, $2, $3) RETURNING *",
            token,
            expiration_time,
            user_id
        )
        .fetch_one(pool)
        .await?)
    }

    // Returns an access token if non-expired is found
    #[instrument(skip(token))]
    pub async fn from_token(
        token: impl ToString + Display + Debug,
        pool: &PgPool,
    ) -> Result<AccessToken, RouteError> {
        info!("Querying potential access token");

        let potential_token = sqlx::query_as!(
            AccessToken,
            "SELECT * FROM access_tokens WHERE token = $1 AND expires > NOW()",
            token.to_string()
        )
        .fetch_optional(pool)
        .await?;

        match potential_token {
            Some(access_token) => Ok(access_token),
            None => {
                warn!("Invalid access token provided");

                Err(RouteError::new(
                    "No valid sessions found.",
                    None::<&str>,
                    StatusCode::FORBIDDEN,
                ))
            }
        }
    }

    #[instrument]
    pub fn is_valid(&self) -> bool {
        if self.expires >= Utc::now() {
            true
        } else {
            false
        }
    }

    // Validity check which returns an error
    #[instrument]
    pub fn is_valid_error(&self) -> Result<(), RouteError> {
        if self.is_valid() {
            Ok(())
        } else {
            warn!("Access token {} expired after is was queried", self.token);

            Err(RouteError::new(
                "Session has expired.",
                None::<&str>,
                StatusCode::FORBIDDEN,
            ))
        }
    }

    // Returns the user bound to the access token
    #[instrument]
    pub async fn get_user(&self, pool: &PgPool) -> Result<User, RouteError> {
        // Just making sure token is still valid
        self.is_valid_error()?;

        Ok(
            sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", self.user_id)
                .fetch_one(pool)
                .await?,
        )
    }

    #[instrument]
    pub async fn delete(self, pool: &PgPool) -> Result<(), RouteError> {
        match sqlx::query!("DELETE FROM access_tokens WHERE token = $1", &self.token)
            .execute(pool)
            .await
        {
            Ok(_) => Ok(()),
            Err(error) => {
                error!(
                    "Failed to delete an existing access token {} which should not happen: {}",
                    &self.token, error
                );

                Err(RouteError::new(
                    "Failed to delete token.",
                    None::<&str>,
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use crate::test_utils::database::create_test_user;

    use super::*;

    #[sqlx::test]
    async fn unexpired(pool: PgPool) {
        let user = create_test_user(&pool).await;

        // Token with 1 minute validity
        let access_token: AccessToken = AccessToken::new(user.id, Duration::seconds(60), &pool)
            .await
            .unwrap();

        // Should still be valid
        assert!(access_token.is_valid());

        assert!(access_token.is_valid_error().is_ok());

        assert!(AccessToken::from_token(access_token.token, &pool)
            .await
            .is_ok());
    }

    #[sqlx::test]
    async fn expired(pool: PgPool) {
        let user = create_test_user(&pool).await;

        // Token with 1 second validity
        let access_token: AccessToken = AccessToken::new(user.id, Duration::seconds(1), &pool)
            .await
            .unwrap();

        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        // 2 seconds have passed, should not be valid anymore
        assert!(!access_token.is_valid());

        assert!(access_token.is_valid_error().is_err());

        assert!(AccessToken::from_token(access_token.token, &pool)
            .await
            .is_err());
    }

    #[sqlx::test]
    async fn deleted_user(pool: PgPool) {
        let user = create_test_user(&pool).await;

        let access_token: AccessToken = AccessToken::new(user.id, Duration::seconds(60), &pool)
            .await
            .unwrap();

        // Delete the user
        user.delete(&pool).await.unwrap();

        // Access token should not exist anymore
        assert!(AccessToken::from_token(access_token.token, &pool)
            .await
            .is_err());
    }

    #[sqlx::test]
    async fn get_user(pool: PgPool) {
        let user = create_test_user(&pool).await;

        let access_token: AccessToken = AccessToken::new(user.id, Duration::seconds(60), &pool)
            .await
            .unwrap();

        // Returned user should be the same which was used to create the access token
        assert_eq!(user, access_token.get_user(&pool).await.unwrap());
    }

    #[sqlx::test]
    async fn delete(pool: PgPool) {
        let user = create_test_user(&pool).await;

        let access_token: AccessToken = AccessToken::new(user.id, Duration::seconds(60), &pool)
            .await
            .unwrap();

        let token_string = access_token.token.clone();

        // Delete token
        access_token.delete(&pool).await.unwrap();

        // Should not exist anymore
        assert!(AccessToken::from_token(token_string, &pool).await.is_err());
    }
}
