use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use chrono::Duration;
use serde::Deserialize;
use sqlx::PgPool;
use utoipa::ToSchema;
use validator::Validate;

use crate::{
    api::{
        extractors::json::ValidatedJson,
        response::{RouteResponse, RouteSuccess},
        routes::user::REGEX_USERNAME,
    },
    models::{access_token::AccessToken, user::User},
};

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct CreateAccessTokenInput {
    #[validate(regex(
        path = "REGEX_USERNAME",
        message = "only letters a-z, A-Z, numbers, - and _ are allowed"
    ))]
    #[schema(example = "some_username")]
    username: String,
    #[validate(length(min = 10, max = 200, message = "must be between 10 and 200 characters"))]
    #[schema(example = "strong_password_with_at_least_10_characters")]
    password: String,
    #[validate(range(
        min = 1,
        max = 2592000,
        message = "validity is limited to 30 days (2592000 seconds)"
    ))]
    #[schema(example = "600")]
    validity_in_seconds: i64,
}

#[utoipa::path(
    post,
    path = "/api/access_token",
    request_body = CreateAccessTokenInput,
    responses(
        (status = OK, description = "New token created", body = RouteSuccessAccessToken),
        (status = UNAUTHORIZED, description = "Wrong password", body = RouteError),
        (status = NOT_FOUND, description = "Username not found", body = RouteError),
        (status = INTERNAL_SERVER_ERROR, description = "Password hashing failed", body = RouteError)
    )
)]
pub async fn create_access_token(
    State(pool): State<PgPool>,
    ValidatedJson(body): ValidatedJson<CreateAccessTokenInput>,
) -> RouteResponse<AccessToken> {
    let user = User::from_credentials(body.username, body.password, &pool).await?;

    let duration = Duration::seconds(body.validity_in_seconds.into());

    Ok(RouteSuccess::new(
        "New access token created.",
        AccessToken::new(user.id, duration, &pool).await?,
        StatusCode::OK,
    ))
}

#[utoipa::path(
    delete,
    path = "/api/access_token/{token}",
    params(
        ("token" = String, Path, description = "The token being deleted")
    ),
    responses(
        (status = OK, description = "Token deleted", body = RouteSuccessString),
        (status = FORBIDDEN, description = "Invalid access token", body = RouteError),
        (status = BAD_REQUEST, description = "Access token missing or malformed", body = RouteError)
    )
)]
pub async fn delete_token(
    Path(token): Path<String>,
    State(pool): State<PgPool>,
) -> RouteResponse<String> {
    AccessToken::from_token(&token, &pool)
        .await?
        .delete(&pool)
        .await?;

    Ok(RouteSuccess::new(
        "Access token deleted.",
        token,
        StatusCode::OK,
    ))
}

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
    async fn create_test_token(server: &TestServer, validity_in_seconds: i64) -> AccessToken {
        // Create a new token using REST API
        let response = server
            .post("/api/access_token")
            .json(&json!({
                "username": "test",
                "password": "testuserpassword",
                "validity_in_seconds": validity_in_seconds
            }))
            .await;

        response.assert_status_success();

        response.json::<RouteSuccess<AccessToken>>().data
    }

    // Create a new access token and use it to get self
    #[sqlx::test]
    async fn create_token(pool: PgPool) {
        let (mut server, user, _) = create_test_app(&pool).await;
        server.clear_headers();

        let new_access_token = create_test_token(&server, 60).await;

        // The new token should work with getting self from API
        let self_response = server
            .get("/api/user")
            .add_header(
                HeaderName::from_static("authorization"),
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
        let (mut server, _, _) = create_test_app(&pool).await;
        server.clear_headers();

        let access_token = create_test_token(&server, 60).await;

        // Delete the token
        let delete_response = server
            .delete(&format!("/api/access_token/{}", access_token.token))
            .await;

        delete_response.assert_status_success();

        // Token should not work anymore
        let invalid_token_response = server
            .get("/api/user")
            .add_header(
                HeaderName::from_static("authorization"),
                HeaderValue::from_bytes(format!("Bearer {}", access_token.token).as_bytes())
                    .unwrap(),
            )
            .await;

        invalid_token_response.assert_status_failure();
    }

    #[sqlx::test]
    async fn expired_token(pool: PgPool) {
        let (mut server, _, _) = create_test_app(&pool).await;
        server.clear_headers();

        // Just one second validity, wait until it expires
        let access_token = create_test_token(&server, 1).await;

        // Wait for token to expire
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        // Try to use token, should not work
        let invalid_token_response = server
            .get("/api/user")
            .add_header(
                HeaderName::from_static("authorization"),
                HeaderValue::from_bytes(format!("Bearer {}", access_token.token).as_bytes())
                    .unwrap(),
            )
            .await;

        invalid_token_response.assert_status_failure();
    }

    #[sqlx::test]
    async fn too_long_validity(pool: PgPool) {
        let (mut server, _, _) = create_test_app(&pool).await;
        server.clear_headers();

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
        let (mut server, _, _) = create_test_app(&pool).await;
        server.clear_headers();

        // Try to query self without a token
        let invalid_token_response = server.get("/api/user").await;

        invalid_token_response.assert_status_failure();
    }

    #[sqlx::test]
    async fn non_exsisting_token(pool: PgPool) {
        let (mut server, _, _) = create_test_app(&pool).await;
        server.clear_headers();

        // Try to query self with a token that doesn't exist
        let invalid_token_response = server
            .get("/api/user")
            .add_header(
                HeaderName::from_static("authorization"),
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
        let (mut server, _, _) = create_test_app(&pool).await;
        server.clear_headers();

        // Try to query self with a token that doesn't exist
        let invalid_token_response = server
            .get("/api/user")
            .add_header(
                HeaderName::from_static("authorization"),
                HeaderValue::from_bytes(
                    format!("Bearer {}", "should not be spaces here").as_bytes(),
                )
                .unwrap(),
            )
            .await;

        invalid_token_response.assert_status_failure();
    }
}
