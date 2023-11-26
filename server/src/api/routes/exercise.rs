#[cfg(test)]
mod tests {
    use axum::http::{HeaderName, HeaderValue};
    use axum_test::TestServer;
    use serde_json::json;
    use sqlx::PgPool;

    use crate::{
        api::response::RouteSuccess,
        models::{access_token::AccessToken, exercise::Exercise, user::User},
        test_utils::api::{create_test_app, get_auth_header},
    };

    async fn create_test_exercise(server: &TestServer, access_token: &AccessToken) -> Exercise {
        todo!();
    }

    #[sqlx::test]
    async fn create_exercise(pool: PgPool) {
        let (server, _, token) = create_test_app(pool).await;
        let (header_name, header_value) = get_auth_header(&token);

        // Required fields
        server
            .post("/api/exercise")
            .add_header(header_name.clone(), header_value.clone())
            .json(&json!(
                {
                    "name": "Bench press",
                    "kind": "BARBELL"
                }
            ))
            .await
            .assert_status_success();

        // All fields
        server
            .post("/api/exercise")
            .add_header(header_name.clone(), header_value.clone())
            .json(&json!(
                {
                    "name": "Bench press",
                    "description": "Flat bench",
                    "notes": "Something ...",
                    "favourite": true,
                    "kind": "BARBELL",

                }
            ))
            .await
            .assert_status_success();
    }

    #[sqlx::test]
    async fn edit_exercise_fields(pool: PgPool) {
        let (server, user, token) = create_test_app(pool).await;

        todo!();
    }

    #[sqlx::test]
    async fn edit_exercise_favourite(pool: PgPool) {
        let (server, user, token) = create_test_app(pool).await;

        todo!();
    }

    #[sqlx::test]
    async fn delete_exercise(pool: PgPool) {
        let (server, user, token) = create_test_app(pool).await;

        todo!();
    }

    #[sqlx::test]
    async fn get_multiple_exercises(pool: PgPool) {
        let (server, user, token) = create_test_app(pool).await;

        todo!();
    }

    #[sqlx::test]
    async fn get_multiple_exercises_by_kind(pool: PgPool) {
        let (server, user, token) = create_test_app(pool).await;

        todo!();
    }
}
