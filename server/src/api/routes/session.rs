use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    api::response::{RouteResponse, RouteSuccess},
    models::{
        session::{self, Session},
        user::User,
    },
};

use super::deserialize_optional_option;

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct CreateSessionInput {
    #[validate(length(min = 1, max = 30, message = "must be between 1 and 30 characters"))]
    name: String,
    #[validate(length(
        min = 1,
        max = 10000,
        message = "must be between 1 and 10000 characters"
    ))]
    description: Option<String>,
}

#[utoipa::path(
    post,
    path = "/api/session",
    request_body = CreateSessionInput,
    security(
        ("access_token"= [])
    ),
    responses(
        (status = CREATED, description = "New session created", body = RouteSuccessSession),
        (status = UNAUTHORIZED, description = "Invalid authorization token", body = RouteError),
        (status = BAD_REQUEST, description = "Invalid input for session", body = RouteError),
    )
)]
pub async fn create_session(
    user: User,
    State(pool): State<PgPool>,
    Json(body): Json<CreateSessionInput>,
) -> RouteResponse<Session> {
    body.validate()?;

    Ok(RouteSuccess::new(
        "New session created.",
        Session::new(user.id, body.name, body.description, &pool).await?,
        StatusCode::CREATED,
    ))
}

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct EditSessionInput {
    #[validate(length(min = 1, max = 30, message = "must be between 1 and 30 characters"))]
    name: Option<String>,
    #[validate(length(
        min = 1,
        max = 10000,
        message = "must be between 1 and 10000 characters"
    ))]
    #[serde(default, deserialize_with = "deserialize_optional_option")]
    #[validate(length(
        min = 1,
        max = 10000,
        message = "must be between 1 and 10000 characters"
    ))]
    description: Option<Option<String>>,
}

#[utoipa::path(
    patch,
    path = "/api/session/{session_id}",
    params(
        ("session_id" = Uuid, Path, description = "The ID of the session edited")
    ),
    request_body = EditSessionInput,
    security(
        ("access_token"= [])
    ),
    responses(
        (status = OK, description = "Session modified", body = RouteSuccessSession),
        (status = UNAUTHORIZED, description = "Invalid authorization token", body = RouteError),
        (status = BAD_REQUEST, description = "Invalid input for session", body = RouteError),
    )
)]
pub async fn edit_session(
    user: User,
    State(pool): State<PgPool>,
    Path(session_id): Path<Uuid>,
    Json(body): Json<EditSessionInput>,
) -> RouteResponse<Session> {
    body.validate()?;

    let mut session = Session::from_id(user.id, session_id, &pool).await?;

    if let Some(new_name) = body.name {
        session.set_name(new_name, &pool).await?
    }

    if let Some(new_descripstion) = body.description {
        session.set_description(new_descripstion, &pool).await?;
    }

    Ok(RouteSuccess::new(
        "Session modified if changes were requested.",
        session,
        StatusCode::OK,
    ))
}

#[utoipa::path(
    delete,
    path = "/api/session/{session_id}",
    params(
        ("session_id" = Uuid, Path, description = "The ID of the session being deleted")
    ),
    security(
        ("access_token"= [])
    ),
    responses(
        (status = OK, description = "Session found and deleted", body = RouteSuccessUuid),
        (status = NOT_FOUND, description = "Invalid session ID", body = RouteError),
        (status = UNAUTHORIZED, description = "Invalid authorization token", body = RouteError),
        (status = BAD_REQUEST, description = "Invalid ID format", body = RouteError),
    )
)]
pub async fn delete_session_by_id(
    user: User,
    State(pool): State<PgPool>,
    Path(session_id): Path<Uuid>,
) -> RouteResponse<Uuid> {
    Ok(RouteSuccess::new(
        "Session and related items deleted.",
        Session::from_id(user.id, session_id, &pool)
            .await?
            .delete(&pool)
            .await?,
        StatusCode::OK,
    ))
}

#[utoipa::path(
    get,
    path = "/api/session/{session_id}",
    params(
        ("session_id" = Uuid, Path, description = "The ID of the session requested")
    ),
    security(
        ("access_token"= [])
    ),
    responses(
        (status = FOUND, description = "Session found", body = RouteSuccessSession),
        (status = NOT_FOUND, description = "Invalid session ID", body = RouteError),
        (status = UNAUTHORIZED, description = "Invalid authorization token", body = RouteError),
        (status = BAD_REQUEST, description = "Invalid ID format", body = RouteError),
    )
)]
pub async fn get_session_by_id(
    user: User,
    State(pool): State<PgPool>,
    Path(session_id): Path<Uuid>,
) -> RouteResponse<Session> {
    Ok(RouteSuccess::new(
        "Session found.",
        Session::from_id(user.id, session_id, &pool).await?,
        StatusCode::FOUND,
    ))
}

#[utoipa::path(
    patch,
    path = "/api/session/{session_id}/finish",
    params(
        ("session_id" = Uuid, Path, description = "The ID of the finished session")
    ),
    security(
        ("access_token"= [])
    ),
    responses(
        (status = OK, description = "Session set as finished", body = RouteSuccessSession),
        (status = NOT_FOUND, description = "Invalid session ID", body = RouteError),
        (status = UNAUTHORIZED, description = "Invalid authorization token", body = RouteError),
        (status = BAD_REQUEST, description = "Invalid ID format", body = RouteError),
    )
)]
pub async fn finish_session(
    user: User,
    State(pool): State<PgPool>,
    Path(session_id): Path<Uuid>,
) -> RouteResponse<Session> {
    let mut session = Session::from_id(user.id, session_id, &pool).await?;

    session.mark_finished(&pool).await?;

    Ok(RouteSuccess::new(
        "Session set as finished.",
        session,
        StatusCode::OK,
    ))
}

#[utoipa::path(
    get,
    path = "/api/session",
    security(
        ("access_token"= [])
    ),
    responses(
        (status = FOUND, description = "Returned all user's sessions (zero or more)", body = RouteSuccessSessionVec),
        (status = UNAUTHORIZED, description = "Invalid authorization token", body = RouteError),
        (status = BAD_REQUEST, description = "Invalid ID format", body = RouteError),
    )
)]
pub async fn get_all_user_sessions(
    user: User,
    State(pool): State<PgPool>,
) -> RouteResponse<Vec<Session>> {
    Ok(RouteSuccess::new(
        "Returned all user's sessions.",
        session::all_user_sessions(user.id, &pool).await?,
        StatusCode::FOUND,
    ))
}

#[cfg(test)]
mod tests {
    use axum_test::TestServer;
    use serde_json::json;
    use sqlx::PgPool;
    use uuid::Uuid;

    use crate::{
        api::response::RouteSuccess,
        models::session::Session,
        test_utils::database::{
            create_test_exercise, create_test_exercise_instance, create_test_scenario,
            create_test_session,
        },
    };

    // Helper to get a session and panic if it doesn't exist
    async fn query_by_id(id: Uuid, server: &TestServer) -> Session {
        server
            .get(&format!("/api/session/{}", id))
            .await
            .json::<RouteSuccess<Session>>()
            .data
    }

    #[sqlx::test]
    async fn create_and_query(pool: PgPool) {
        let (server, _, _, _, session, _, _) = create_test_scenario(&pool).await;

        // Query for the session
        let query = server
            .get(&format!("/api/session/{}", session.id))
            .await
            .json::<RouteSuccess<Session>>()
            .data;

        assert_eq!(query, session);

        // Invalid ID
        let invalid_query = server
            .get(&format!("/api/session/{}", Uuid::new_v4()))
            .await;

        invalid_query.assert_status_failure();
    }

    #[sqlx::test]
    async fn set_name(pool: PgPool) {
        let (server, _, _, _, session, _, _) = create_test_scenario(&pool).await;

        let new_name = "A new name";

        // Change the name
        server
            .patch(&format!("/api/session/{}", session.id))
            .json(&json!({
                "name": new_name
            }))
            .await
            .assert_status_success();

        let query = query_by_id(session.id, &server).await;

        assert_eq!(query.name, new_name.to_string());
        assert_eq!(session.description, query.description);
    }

    // Try to delete with at least one instance linked
    #[sqlx::test]
    async fn delete_with_exercise_instances(pool: PgPool) {
        let (server, _, _, _, session, _, _) = create_test_scenario(&pool).await;

        // Add a exercise
        let exercise = create_test_exercise(&server).await;

        // Add an instance
        let exercise_instance =
            create_test_exercise_instance(&server, session.id, exercise.id).await;

        // Delete session, should work
        server
            .delete(&format!("/api/session/{}", session.id))
            .await
            .assert_status_success();

        // Exercise instance should not exist anymore
        server
            .get(&format!("/api/exercise_instance/{}", exercise_instance.id))
            .await
            .assert_status_failure();
    }

    #[sqlx::test]
    async fn set_description(pool: PgPool) {
        let (server, _, _, _, session, _, _) = create_test_scenario(&pool).await;

        let new_description: Option<&str> = None;

        // Remove the description with None variant
        server
            .patch(&format!("/api/session/{}", session.id))
            .json(&json!({
                "description": new_description
            }))
            .await
            .assert_status_success();

        let query = query_by_id(session.id, &server).await;

        assert!(query.description.is_none());
        assert_eq!(session.name, query.name);
    }

    #[sqlx::test]
    async fn finish(pool: PgPool) {
        let (server, _, _, _, session, _, _) = create_test_scenario(&pool).await;

        server
            .patch(&format!("/api/session/{}/finish", session.id))
            .await
            .assert_status_success();

        assert!(query_by_id(session.id, &server).await.finished.is_some());
    }

    // Get all sessions of an users
    #[sqlx::test]
    async fn get_all(pool: PgPool) {
        let (server, _, _, _, session, _, _) = create_test_scenario(&pool).await;

        let mut test_sessions = vec![session];

        for _ in 1..10 {
            test_sessions.push(create_test_session(&server).await);
        }

        let all_sessions = server
            .get("/api/session")
            .await
            .json::<RouteSuccess<Vec<Session>>>()
            .data;

        assert_eq!(test_sessions.len(), all_sessions.len());
    }
}
