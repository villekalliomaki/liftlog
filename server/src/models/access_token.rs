use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

// Used to authenticate single API requests to a specific user
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct AccessToken {
    token: String,
    created: DateTime<Utc>,
    expires: DateTime<Utc>,
    user_id: Uuid,
}

#[cfg(test)]
mod tests {
    use chrono::Duration;
    use serial_test::serial;

    use crate::test_utils::database::{clean_sqlx_pool, test_user};

    use super::*;

    #[tokio::test]
    #[serial]
    async fn unexpired() {
        let pool = &clean_sqlx_pool().await;
        let user = test_user(pool).await;

        // Token with 1 minute validity
        let access_token: AccessToken = AccessToken::new(user.id, Duration::seconds(60), pool)
            .await
            .unwrap();

        // Should still be valid
        assert!(AccessToken::from_token(access_token.token, pool)
            .await
            .is_ok());
    }

    #[tokio::test]
    #[serial]
    async fn expired() {
        let pool = &clean_sqlx_pool().await;
        let user = test_user(pool).await;

        // Token with 1 second validity
        let access_token: AccessToken = AccessToken::new(user.id, Duration::seconds(1), pool)
            .await
            .unwrap();

        tokio::time::sleep(Duration::seconds(2)).await;

        // 2 seconds have passed, should not be valid anymore
        assert!(AccessToken::from_token(access_token.token, pool)
            .await
            .is_err());
    }

    #[tokio::test]
    #[serial]
    async fn deleted_user() {
        let pool = &clean_sqlx_pool().await;
        let user = test_user(pool).await;

        let access_token: AccessToken = AccessToken::new(user.id, Duration::seconds(60), pool)
            .await
            .unwrap();

        // Delete the user
        user.delete(pool).await.unwrap();

        // Access token should not exist anymore
        assert!(AccessToken::from_token(access_token.token, pool)
            .await
            .is_err());
    }

    #[tokio::test]
    #[serial]
    async fn get_user() {
        let pool = &clean_sqlx_pool().await;
        let user = test_user(pool).await;

        let access_token: AccessToken = AccessToken::new(user.id, Duration::seconds(60), pool)
            .await
            .unwrap();

        // Returned user should be the same which was used to create the access token
        assert_eq!(user, access_token.get_user(pool));
    }

    #[tokio::test]
    #[serial]
    async fn delete() {
        let pool = &clean_sqlx_pool().await;
        let user = test_user(pool).await;

        let access_token: AccessToken = AccessToken::new(user.id, Duration::seconds(60), pool)
            .await
            .unwrap();

        let token_string = access_token.token.clone();

        // Delete token
        access_token.delete(pool).await.unwrap();

        // Should not exist anymore
        assert!(AccessToken::from_token(token_string, pool).await.is_err());
    }
}
