use axum::http::{HeaderName, HeaderValue};
use axum_test::TestServer;
use sqlx::PgPool;

use crate::{
    api::routes::build_router,
    models::{access_token::AccessToken, user::User},
};

use super::database::create_test_access_token;

// Shorter way to create a test server in integration tests
pub fn test_server(pool: &PgPool) -> TestServer {
    TestServer::new(build_router(pool.to_owned()).into_make_service()).unwrap()
}

// Create a test server, access token and an user
pub async fn create_test_app(pool: &PgPool) -> (TestServer, User, AccessToken) {
    let (user, access_token) = create_test_access_token(pool).await;

    let mut server = test_server(pool);

    // Add auth headers to they don't have to me manually added each time
    let (header_name, header_value) = get_auth_header(&access_token);
    server.add_header(header_name, header_value);

    (server, user, access_token)
}

pub fn get_auth_header(access_token: &AccessToken) -> (HeaderName, HeaderValue) {
    (
        HeaderName::from_static("authorization"),
        HeaderValue::from_bytes(format!("Bearer {}", access_token.token).as_bytes()).unwrap(),
    )
}
