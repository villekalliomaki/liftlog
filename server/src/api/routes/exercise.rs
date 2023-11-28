
#[cfg(test)]
mod tests {
    use axum::http::{HeaderName, HeaderValue};
    use axum_test::TestServer;
    use serde_json::json;
    use sqlx::PgPool;

    use crate::{
        api::response::RouteSuccess,
        models::exercise::{Exercise, ExerciseKind},
        test_utils::api::{create_test_app, get_auth_header},
    };

    async fn create_test_exercise(
        server: &TestServer,
        access_token_header_name: &HeaderName,
        access_token_header_value: &HeaderValue,
    ) -> Exercise {
        server
            .post("/api/exercise")
            .add_header(
                access_token_header_name.clone(),
                access_token_header_value.clone(),
            )
            .json(&json!(
                {
                    "name": "Bench press",
                    "description": "Flat bench",
                    "notes": "Something ...",
                    "favourite": true,
                    "kind": "barbell",

                }
            ))
            .await
            .json::<RouteSuccess<Exercise>>()
            .data
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
                    "kind": "barbell"
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
                    "kind": "barbell",

                }
            ))
            .await
            .assert_status_success();
    }

    #[sqlx::test]
    async fn edit_exercise_fields(pool: PgPool) {
        let (server, _, token) = create_test_app(pool).await;
        let (header_name, header_value) = get_auth_header(&token);
        let test_exercise = create_test_exercise(&server, &header_name, &header_value).await;

        let new_name = "Lateral Row";
        let new_description: Option<String> = None;
        let new_notes: Option<String> = Some("Something else".into());
        let new_kind = ExerciseKind::Cable;

        // Update all
        server
            .patch(&format!("/api/exercise/{}", test_exercise.id))
            .add_header(header_name.clone(), header_value.clone())
            .json(&json!(
                {
                    "name": new_name,
                    "description": new_description,
                    "notes": new_notes,
                    "favourite": false,
                    "kind": new_kind
                }
            ))
            .await;

        // Check that they are updated
        let updated_test_exercise = server
            .get(&format!("/api/exercise/{}", test_exercise.id))
            .add_header(header_name.clone(), header_value.clone())
            .await
            .json::<RouteSuccess<Exercise>>()
            .data;

        assert_eq!(updated_test_exercise.name, new_name);
        assert_eq!(updated_test_exercise.description, new_description);
        assert_eq!(updated_test_exercise.notes, new_notes);
        assert!(!updated_test_exercise.favourite);
        assert_eq!(updated_test_exercise.kind, new_kind);
    }

    #[sqlx::test]
    async fn edit_exercise_fields_partial(pool: PgPool) {
        let (server, _, token) = create_test_app(pool).await;
        let (header_name, header_value) = get_auth_header(&token);
        let test_exercise = create_test_exercise(&server, &header_name, &header_value).await;

        let new_description: Option<String> = None;

        // Just one, should still work and parse it correctly, while only updating the one field included keeping others
        server
            .patch(&format!("/api/exercise/{}", test_exercise.id))
            .add_header(header_name.clone(), header_value.clone())
            .json(&json!(
                {
                    "description": new_description,
                }
            ))
            .await;

        // Check that they are updated
        let updated_test_exercise = server
            .get(&format!("/api/exercise/{}", test_exercise.id))
            .add_header(header_name.clone(), header_value.clone())
            .await
            .json::<RouteSuccess<Exercise>>()
            .data;

        // Changed
        assert_eq!(updated_test_exercise.description, new_description);

        // Unchanged
        assert_eq!(updated_test_exercise.name, test_exercise.name);
        assert_eq!(updated_test_exercise.notes, test_exercise.notes);
        assert_eq!(!updated_test_exercise.favourite, test_exercise.favourite);
        assert_eq!(updated_test_exercise.kind, test_exercise.kind);
    }

    #[sqlx::test]
    async fn delete_exercise(pool: PgPool) {
        let (server, _, token) = create_test_app(pool).await;
        let (header_name, header_value) = get_auth_header(&token);
        let test_exercise = create_test_exercise(&server, &header_name, &header_value).await;

        server
            .delete(&format!("/api/exercise/{}", test_exercise.id))
            .add_header(header_name.clone(), header_value.clone())
            .await
            .assert_status_success();

        server
            .get(&format!("/api/exercise/{}", test_exercise.id))
            .add_header(header_name.clone(), header_value.clone())
            .await
            .assert_status_failure()
    }

    #[sqlx::test]
    async fn get_multiple_exercises(pool: PgPool) {
        let (server, _, token) = create_test_app(pool).await;
        let (header_name, header_value) = get_auth_header(&token);

        for _ in 1..20 {
            create_test_exercise(&server, &header_name, &header_value).await;
        }

        // Note that this is /exercises not /exercise
        let users_exercises = server
            .get("/api/exercises")
            .add_header(header_name.clone(), header_value.clone())
            .await
            .json::<RouteSuccess<Vec<Exercise>>>();

        // There should be 20
        assert_eq!(users_exercises.data.len(), 20);
    }

    #[sqlx::test]
    async fn get_multiple_exercises_by_kind(pool: PgPool) {
        let (server, _, token) = create_test_app(pool).await;
        let (header_name, header_value) = get_auth_header(&token);

        // One barbell kind
        let barbell_exercise = server
            .post("/api/exercise")
            .add_header(header_name.clone(), header_value.clone())
            .json(&json!(
                {
                    "name": "Bench press",
                    "kind": "barbell"
                }
            ))
            .await
            .json::<RouteSuccess<Exercise>>()
            .data;

        // One cable kind
        server
            .post("/api/exercise")
            .add_header(header_name.clone(), header_value.clone())
            .json(&json!(
                {
                    "name": "Lat pulldown",
                    "kind": "cable"
                }
            ))
            .await
            .assert_status_success();

        // Note that this is /exercises not /exercise
        let users_exercises = server
            .get("/api/exercises/barbell")
            .add_header(header_name.clone(), header_value.clone())
            .await
            .json::<RouteSuccess<Vec<Exercise>>>()
            .data;

        // There should be just the one
        assert_eq!(users_exercises.len(), 1);
        assert_eq!(users_exercises.get(0).unwrap().id, barbell_exercise.id);
    }
}
