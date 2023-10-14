#[cfg(test)]
mod tests {
    use axum::http::{HeaderName, HeaderValue};
    use axum_test::TestServer;
    use serde_json::json;
    use sqlx::PgPool;

    use crate::{
        api::response::RouteSuccess,
        models::{access_token::AccessToken, user::User},
        test_utils::api::create_test_app,
    };

    // A new access token to the test user which is valid for a minute
    async fn create_test_token(server: &TestServer, validity_in_seconds: u64) -> AccessToken {
        // Create a new token using REST API
        let response = server
            .post("/api/access_token")
            .json(&json!({
                "username": "test",
                "password": "test",
                "validity_in_seconds": validity_in_seconds
            }))
            .await;

        response.assert_status_success();

        response.json::<RouteSuccess<AccessToken>>().data
    }

    // Create a new access token and use it to get self
    #[sqlx::test]
    async fn create_token(pool: PgPool) {
        let (server, user, _) = create_test_app(pool).await;

        let new_access_token = create_test_token(&server, 60).await;

        // The new token should work with getting self from API
        let self_response = server
            .get("/api/user")
            .add_header(
                HeaderName::from_static("Authorization"),
                HeaderValue::from_bytes(format!("Bearer {}", new_access_token.token).as_bytes())
                    .unwrap(),
            )
            .await;

        self_response.assert_status_success();

        let queried_user = self_response.json::<RouteSuccess<User>>().data;

        assert_eq!(queried_user.id, user.id);
    }

    #[sqlx::test]
    async fn delete_token(pool: PgPool) {
        let (server, _, _) = create_test_app(pool).await;
        let access_token = create_test_token(&server, 60).await;

        // Delete the token
        let delete_response = server
            .delete("/api/access_token")
            .json(&json! ({"access_token": access_token.token}))
            .await;

        delete_response.assert_status_success();

        // Token should not work anymore
        let invalid_token_response = server
            .get("/api/user")
            .add_header(
                HeaderName::from_static("Authorization"),
                HeaderValue::from_bytes(format!("Bearer {}", access_token.token).as_bytes())
                    .unwrap(),
            )
            .await;

        invalid_token_response.assert_status_failure();
    }

    #[sqlx::test]
    async fn expired_token(pool: PgPool) {
        let (server, _, _) = create_test_app(pool).await;
        // Just one second validity, wait until it expires
        let access_token = create_test_token(&server, 1).await;

        // Wait for token to expire
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        // Try to use token, should not work
        let invalid_token_response = server
            .get("/api/user")
            .add_header(
                HeaderName::from_static("Authorization"),
                HeaderValue::from_bytes(format!("Bearer {}", access_token.token).as_bytes())
                    .unwrap(),
            )
            .await;

        invalid_token_response.assert_status_failure();
    }

    #[sqlx::test]
    async fn too_long_validity(pool: PgPool) {
        let (server, _, _) = create_test_app(pool).await;

        // 2592000 is 30 days, so lets try 31
        let response = server
            .post("/api/access_token")
            .json(&json!({
                "username": "test",
                "password": "test",
                "validity_in_seconds": 2678400
            }))
            .await;

        response.assert_status_failure();
    }

    #[sqlx::test]
    async fn missing_token(pool: PgPool) {
        let (server, _, _) = create_test_app(pool).await;

        // Try to query self without a token
        let invalid_token_response = server.get("/api/user").await;

        invalid_token_response.assert_status_failure();
    }

    #[sqlx::test]
    async fn non_exsisting_token(pool: PgPool) {
        let (server, _, _) = create_test_app(pool).await;

        // Try to query self with a token that doesn't exist
        let invalid_token_response = server
            .get("/api/user")
            .add_header(
                HeaderName::from_static("Authorization"),
                HeaderValue::from_bytes(
                    format!("Bearer {}", "this_is_not_a_valid_token").as_bytes(),
                )
                .unwrap(),
            )
            .await;

        invalid_token_response.assert_status_failure();
    }

    #[sqlx::test]
    async fn wrong_token_format(pool: PgPool) {
        let (server, _, _) = create_test_app(pool).await;

        // Try to query self with a token that doesn't exist
        let invalid_token_response = server
            .get("/api/user")
            .add_header(
                HeaderName::from_static("Authorization"),
                HeaderValue::from_bytes(
                    format!("Bearer {}", "should not be spaces here").as_bytes(),
                )
                .unwrap(),
            )
            .await;

        invalid_token_response.assert_status_failure();
    }
}
