use chrono::Duration;
use sqlx::PgPool;

use crate::models::{user::User, access_token::AccessToken};

// Create a test user
pub async fn test_user(pool: &PgPool) -> User {
    User::new("test", "testuserpassword", pool).await.unwrap()
}

// A test token based on the test user, return both
pub async fn test_access_token(pool: &PgPool) -> (User, AccessToken) {
    let user = test_user(pool).await;

    let access_token = AccessToken::new(user.id, Duration::days(1), pool).await.unwrap();

    (user, access_token)
}