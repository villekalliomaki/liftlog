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
