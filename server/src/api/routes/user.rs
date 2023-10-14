use axum::{extract::State, http::StatusCode, Json};
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

use crate::{
    api::response::{RouteResponse, RouteSuccess},
    models::user::User,
};

lazy_static! {
    static ref REGEX_USERNAME: Regex = Regex::new(r"^[a-zA-Z0-9_-]{1,20}$").unwrap();
}

#[derive(Debug, Validate, Deserialize)]
pub struct CreateUserInput {
    #[validate(regex(
        path = "REGEX_USERNAME",
        message = "only letters a-z, A-Z, numbers, - and _ are allowed"
    ))]
    username: String,
    #[validate(length(min = 10, max = 200, message = "must be between 10 and 200 characters"))]
    password: String,
}

// New user from the username and password provided
pub async fn create_user(
    State(pool): State<PgPool>,
    Json(body): Json<CreateUserInput>,
) -> RouteResponse<User> {
    body.validate()?;

    Ok(RouteSuccess::new(
        "New user created.",
        User::new(body.username, body.password, &pool).await?,
        StatusCode::OK,
    ))
}

#[cfg(test)]
mod tests {
    use axum::http::{HeaderName, HeaderValue, StatusCode};
    use axum_test::TestServer;
    use serde_json::json;
    use sqlx::PgPool;

    use crate::{
        api::response::RouteSuccess,
        models::{access_token::AccessToken, user::User},
        test_utils::api::test_server,
    };

    const PASSWORD: &str = "3pWi7ttGSVVWLj";
    const USERNAME: &str = "test_username";

    // Create a test user from the values above
    async fn test_user_from_api(server: &TestServer) -> User {
        server
            .post("/api/user")
            .json(&json!({
                "username": USERNAME,
                "password": PASSWORD
            }))
            .await
            .json::<RouteSuccess<User>>()
            .data
    }

    // Get an access token from the for the test user
    // Assumes password or username has not been changedu
    async fn access_token_from_api(server: &TestServer) -> AccessToken {
        server
            .post("/api/access_token")
            .json(&json!({
                "username": USERNAME,
                "password": PASSWORD
            }))
            .await
            .json::<RouteSuccess<AccessToken>>()
            .data
    }

    #[sqlx::test]
    async fn create_user(pool: PgPool) {
        let server = test_server(pool);

        let _ = test_user_from_api(&server).await;
        let access_token = access_token_from_api(&server).await;

        // Should be able to get self with an access token
        let get_self = server
            .post("/api/user")
            .add_header(
                HeaderName::from_static("Authorization"),
                HeaderValue::from_bytes(format!("Bearer {}", access_token.token).as_bytes())
                    .unwrap(),
            )
            .await
            .json::<RouteSuccess<User>>()
            .data;

        assert_eq!(get_self.username, USERNAME);
    }

    #[sqlx::test]
    async fn username_conflict(pool: PgPool) {
        let server = test_server(pool);

        let response = server
            .post("/api/user")
            .json(&json!({
                "username": USERNAME,
                "password": PASSWORD
            }))
            .await;

        // Empty database, should be ok
        response.assert_status(StatusCode::OK);

        // Try to create an user again
        let response2 = server
            .post("/api/user")
            .json(&json!({
                "username": USERNAME,
                "password": PASSWORD
            }))
            .await;

        response2.assert_status_failure();
    }

    #[sqlx::test]
    async fn delete_user(pool: PgPool) {
        let server = test_server(pool);

        let _ = test_user_from_api(&server).await;
        let access_token = access_token_from_api(&server).await;

        let delete_response = server
            .delete("/api/user")
            .add_header(
                HeaderName::from_static("Authorization"),
                HeaderValue::from_bytes(format!("Bearer {}", access_token.token).as_bytes())
                    .unwrap(),
            )
            .await;

        // Should be successfully deleted
        delete_response.assert_status_ok();

        // Make sure is deleted
        let access_token_invalid = server
            .post("/api/access_token")
            .json(&json!({
                "username": USERNAME,
                "password": PASSWORD
            }))
            .await;

        // "Login" should not work anymore
        access_token_invalid.assert_status_failure();
    }

    #[sqlx::test]
    async fn change_password(pool: PgPool) {
        let server = test_server(pool);

        let _ = test_user_from_api(&server).await;
        let access_token = access_token_from_api(&server).await;

        let new_password = "w2EFovWuLT5qFGFCtbKykpMUnEFVpGEfNaTtjWnz";

        // Change the password to a different one
        let password_change_response = server
            .patch("/api/user")
            .json(&json!({"new_password": new_password}))
            .add_header(
                HeaderName::from_static("Authorization"),
                HeaderValue::from_bytes(format!("Bearer {}", access_token.token).as_bytes())
                    .unwrap(),
            )
            .await;

        // Should be successful
        password_change_response.assert_status_ok();

        // Try to use the old password, should fail
        let access_token_invalid = server
            .post("/api/access_token")
            .json(&json!({
                "username": USERNAME,
                "password": PASSWORD
            }))
            .await;

        access_token_invalid.assert_status_failure();

        // Try the new password, should work
        let access_token_valid = server
            .post("/api/access_token")
            .json(&json!({
                "username": USERNAME,
                "password": new_password
            }))
            .await;

        access_token_valid.assert_status_ok();
    }

    #[sqlx::test]
    async fn change_username(pool: PgPool) {
        let server = test_server(pool);

        let _ = test_user_from_api(&server).await;
        let access_token = access_token_from_api(&server).await;

        let new_username = "some_new_username";

        // Change the password to a different one
        let username_change_response = server
            .patch("/api/user")
            .json(&json!({"new_username": new_username}))
            .add_header(
                HeaderName::from_static("Authorization"),
                HeaderValue::from_bytes(format!("Bearer {}", access_token.token).as_bytes())
                    .unwrap(),
            )
            .await;

        username_change_response.assert_status_ok();

        // Get self from api
        let get_self = server
            .get("/api/user")
            .add_header(
                HeaderName::from_static("Authorization"),
                HeaderValue::from_bytes(format!("Bearer {}", access_token.token).as_bytes())
                    .unwrap(),
            )
            .await
            .json::<RouteSuccess<User>>()
            .data;

        assert_eq!(get_self.username, new_username);
    }
}
