#[cfg(test)]
mod tests {
    use axum_test::TestServer;
    use serde_json::json;
    use sqlx::PgPool;
    use uuid::Uuid;

    use crate::{
        api::response::RouteSuccess,
        models::{
            exercise::Exercise, exercise_instance::ExerciseInstance, session::Session, set::Set,
            user::User,
        },
        test_utils::api::{create_test_app, get_auth_header},
    };

    // Helper function to initialize a test scenario
    async fn init_test_case(
        pool: &PgPool,
    ) -> (TestServer, User, Session, Exercise, ExerciseInstance) {
        let (mut server, user, access_token) = create_test_app(pool.clone()).await;
        let (header_name, header_value) = get_auth_header(&access_token);

        server.add_header(header_name.clone(), header_value.clone());

        let session = create_test_session(&server).await;
        let exercise = create_test_exercise(&server).await;

        let exercise_instance =
            create_test_exercise_instance(&server, session.id, exercise.id).await;

        (server, user, session, exercise, exercise_instance)
    }

    // Create a test instance
    pub async fn create_test_exercise_instance(
        server: &TestServer,
        session_id: Uuid,
        exercise_id: Uuid,
    ) -> ExerciseInstance {
        server
            .post("/api/exercise_instance")
            .json(&json!(
                {
                    "session_id": session_id,
                    "exercise_id": exercise_id,
                }
            ))
            .await
            .json::<RouteSuccess<ExerciseInstance>>()
            .data
    }

    // Create an empty test session
    async fn create_test_session(server: &TestServer) -> Session {
        server
            .post("/api/session")
            .json(&json!(
                {
                    "name": "Legs",
                    "description": "A description",
                }
            ))
            .await
            .json::<RouteSuccess<Session>>()
            .data
    }

    // Create a test exercise to be linked to any instances
    async fn create_test_exercise(server: &TestServer) -> Exercise {
        server
            .post("/api/exercise")
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
    async fn create_and_query(pool: PgPool) {
        let (server, _, _, _, exercise_instance) = init_test_case(&pool).await;

        // Query instance
        let valid_query = server
            .get(&format!("/api/exercise_instance/{}", exercise_instance.id))
            .await
            .json::<RouteSuccess<ExerciseInstance>>()
            .data;

        assert_eq!(exercise_instance, valid_query);

        // Invalid ID
        let invalid = server
            .get(&format!("/api/exercise_instance/{}", Uuid::new_v4()))
            .await;

        invalid.assert_status_failure();
    }

    #[sqlx::test]
    async fn edit_comments(pool: PgPool) {
        let (server, _, _, _, exercise_instance) = init_test_case(&pool).await;

        let comments = vec!["Comment 1", "Comment 2"];

        let edited = server
            .patch(&format!("/api/exercise_instance/{}", exercise_instance.id))
            .json(&json!({"comments": comments}))
            .await
            .json::<RouteSuccess<ExerciseInstance>>()
            .data;

        assert_eq!(
            edited.comments,
            // Could work just fine comparing String to &str
            comments
                .into_iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
        );
    }

    // Try to change to an invalid exercise
    #[sqlx::test]
    async fn change_exercise(pool: PgPool) {
        let (server, _, _, _, exercise_instance) = init_test_case(&pool).await;

        server
            .patch(&format!("/api/exercise_instance/{}", exercise_instance.id))
            .json(&json!({"exercise_id": Uuid::new_v4()}))
            .await
            .assert_status_failure();
    }

    // Delete with sets linked to the instance
    #[sqlx::test]
    async fn delete_with_sets(pool: PgPool) {
        let (server, _, _, _, exercise_instance) = init_test_case(&pool).await;

        // Create a few sets
        let mut sets: Vec<Set> = vec![];

        for _ in 1..3 {
            sets.push(
                server
                    .post(&format!("/api/set/{}", exercise_instance.id))
                    .await
                    .json::<RouteSuccess<Set>>()
                    .data,
            )
        }

        // Delete instance
        server
            .delete(&format!("/api/exercise_instance/{}", exercise_instance.id))
            .await
            .assert_status_success();

        // Sets should not exist anymore
        for set in sets {
            server
                .get(&format!("/api/set/{}", set.id))
                .await
                .assert_status_failure();
        }
    }

    // Try to delete an exercise which is used in an instance
    #[sqlx::test]
    async fn try_delete_used_exercise(pool: PgPool) {
        let (_, _, _, exercise, _) = init_test_case(&pool).await;

        assert!(exercise.delete(&pool).await.is_err());
    }

    #[sqlx::test]
    async fn get_all_for_session(pool: PgPool) {
        let (server, _, session, exercise, _) = init_test_case(&pool).await;

        // Create a few more instances
        for _ in 1..9 {
            create_test_exercise_instance(&server, session.id, exercise.id).await;
        }

        // Get all in this session
        let all = server
            .get(&format!("/api/exercise_instance/session/{}", session.id))
            .await
            .json::<RouteSuccess<Vec<ExerciseInstance>>>()
            .data;

        assert_eq!(all.len(), 10);
    }
}
