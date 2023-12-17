use axum_test::TestServer;
use chrono::Duration;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    api::response::RouteSuccess,
    models::{
        access_token::AccessToken, exercise::Exercise, exercise_instance::ExerciseInstance,
        session::Session, set::Set, user::User,
    },
};

use super::api::create_test_app;

// Complete test scenario
pub async fn create_test_scenario(
    pool: &PgPool,
) -> (
    TestServer,
    User,
    AccessToken,
    Exercise,
    Session,
    ExerciseInstance,
    Set,
) {
    let (server, user, access_token) = create_test_app(pool).await;

    let exercise = create_test_exercise(&server).await;
    let mut session = create_test_session(&server).await;
    let mut exercise_instance =
        create_test_exercise_instance(&server, session.id, exercise.id).await;
    let set = create_test_set(&server, exercise_instance.id).await;

    // These have to uppdated manually because sets and instances were added after creating these
    session = Session::from_id(user.id, session.id, &pool).await.unwrap();
    exercise_instance = ExerciseInstance::from_id(user.id, exercise_instance.id, &pool).await.unwrap();

    (
        server,
        user,
        access_token,
        exercise,
        session,
        exercise_instance,
        set,
    )
}

// Create a test user
pub async fn create_test_user(pool: &PgPool) -> User {
    User::new("test", "testuserpassword", pool).await.unwrap()
}

// A test token based on the test user, return both
pub async fn create_test_access_token(pool: &PgPool) -> (User, AccessToken) {
    let user = create_test_user(pool).await;

    let access_token = AccessToken::new(user.id, Duration::days(1), pool)
        .await
        .unwrap();

    (user, access_token)
}

pub async fn create_test_exercise(server: &TestServer) -> Exercise {
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

// Create an empty test session
pub async fn create_test_session(server: &TestServer) -> Session {
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

pub async fn create_test_set(server: &TestServer, exercise_instance_id: Uuid) -> Set {
    server
        .post("/api/set")
        .json(&json!({"exercise_instance_id": exercise_instance_id}))
        .await
        .json::<RouteSuccess<Set>>()
        .data
}
