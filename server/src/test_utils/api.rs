use axum_test::TestServer;
use sqlx::PgPool;

use crate::{
    api::routes::build_router,
    models::{access_token::AccessToken, user::User},
};

use super::database::test_access_token;

// Shorter way to create a test server in integration tests
pub fn test_server(pool: PgPool) -> TestServer {
    TestServer::new(build_router(pool).into_make_service()).unwrap()
}

// Create a test server, access token and an user
pub async fn test_app(pool: PgPool) -> (TestServer, User, AccessToken) {
    let user_and_token = test_access_token(&pool).await;

    let server = test_server(pool);

    (server, user_and_token.0, user_and_token.1)
}
