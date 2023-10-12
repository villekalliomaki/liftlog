use crate::api::response::RouteSuccess;
use axum::http::StatusCode;

#[utoipa::path(
    get,
    path = "/api/ping",
    responses(
        (status = 200, description = "Ping", body = RouteSuccessString)
    )
)]
pub async fn handle() -> RouteSuccess<String> {
    RouteSuccess::new("Pong.", "pong".to_string(), StatusCode::OK)
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use sqlx::PgPool;

    use crate::api::routes::build_router;

    #[sqlx::test]
    async fn ping(pool: PgPool) {
        let server = TestServer::new(build_router(pool).into_make_service()).unwrap();

        let response = server.get("/api/ping").await;

        assert_eq!(response.status_code(), StatusCode::OK);
    }
}
