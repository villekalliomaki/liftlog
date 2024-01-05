use axum::http::StatusCode;

use crate::api::response::RouteError;

pub async fn fallback404() -> RouteError {
    RouteError::new("Route not found.", None::<&str>, StatusCode::NOT_FOUND)
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

        let response = server.get("/route/that/doesnt/exist").await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }
}
