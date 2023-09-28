use chrono::{DateTime, Utc};
use serde::Serialize;

use super::user::User;

// Used to authenticate single API requests to a specific user
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct AccessToken {
    token: String,
    created: DateTime<Utc>,
    expires: DateTime<Utc>,
    user: User,
}

#[cfg(test)]
mod tests {
    use super::*;
}
