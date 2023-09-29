use sqlx::PgPool;

use crate::models::user::User;

// Create a test user
pub async fn test_user(pool: &PgPool) -> User {
    User::new("test", "test", pool).await.unwrap()
}
