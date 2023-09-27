use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

// User for authentication and authorization
#[derive(Serialize, Debug, Clone, PartialEq)]
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
    password_hash: String,
}

// User tests interact with the database, so there are helper functions to prepare it
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{api::response::RouteError, test_utils};

    const PASSWORD: &str = "3pWi7ttGSVVWLj";
    const USERNAME: &str = "test_username";

    #[tokio::test]
    async fn username_conflict() {
        let pool = test_utils::database::clean_sqlx_pool().await;

        let user1: Result<User, RouteError> = User::new(USERNAME, PASSWORD, pool).await;

        // First user should be fine
        assert!(user1.is_ok());

        let user2: Result<User, RouteError> = User::new(USERNAME, PASSWORD, pool).await;

        // Should not have been created
        assert!(user2.is_err());
    }

    #[tokio::test]
    async fn changed_column_update() {
        let pool = test_utils::database::clean_sqlx_pool().await;

        let user: User =  User::new(USERNAME, PASSWORD, pool).await.unwrap();
        let initial_changed_column = user.changed.clone();

        // Change username
        user.change_username("new_test_username", pool).await.unwrap();

        // Field should have changed
        assert_ne!(initial_changed_column, user.changed);

        // Also database should have updated
        let user_from_db: User = User::from_credentials(USERNAME, PASSWORD).await.unwrap();

        assert_ne!(initial_changed_column, user_from_db.changed);
    }

    #[tokio::test]
    async fn valid_password() {
        let pool = test_utils::database::clean_sqlx_pool().await;

        let user: User =  User::new(USERNAME, PASSWORD, pool).await.unwrap();

        // Get user from database with the same password
        assert!(User::from_credentials(USERNAME, PASSWORD).await.is_ok());
    }

    #[tokio::test]
    async fn invalid_password() {
        let pool = test_utils::database::clean_sqlx_pool().await;

        let user: User =  User::new(USERNAME, PASSWORD, pool).await.unwrap();

        // Get user from database with a wrong password
        assert!(User::from_credentials(USERNAME, "some_other_password").await.is_err());
    }

    #[tokio::test]
    async fn password_change() {
        let pool = test_utils::database::clean_sqlx_pool().await;

        let user: User =  User::new(USERNAME, PASSWORD, pool).await.unwrap();

        let new_password = "some_other_password_2";

        // Change the password of the user
        assert!(user.change_password(new_password).await.is_ok());

        // Try the old password, should not work
        assert!(User::from_credentials(USERNAME, PASSWORD).await.is_err());

        // Try new pasword, should work
        assert!(User::from_credentials(USERNAME, new_password).await.is_ok());
    }
}
