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
    models::{exercise_instance::ExerciseInstance, user::User},
};

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateExerciseInstanceInput {
    session_id: Uuid,
    exercise_id: Uuid,
}

#[utoipa::path(
    post,
    path = "/api/exercise_instance",
    request_body = CreateExerciseInstanceInput,
    security(
        ("access_token"= [])
    ),
    responses(
        (status = CREATED, description = "New exercise instance created", body = RouteSuccessExerciseInstance),
        (status = UNAUTHORIZED, description = "Invalid authorization token", body = RouteError),
        (status = BAD_REQUEST, description = "Invalid input for exercise instance", body = RouteError),
    )
)]
pub async fn create_exercise_instance(
    user: User,
    State(pool): State<PgPool>,
    Json(body): Json<CreateExerciseInstanceInput>,
) -> RouteResponse<ExerciseInstance> {
    Ok(RouteSuccess::new(
        "New exercise instance created.",
        ExerciseInstance::new(user.id, body.session_id, body.exercise_id, &pool).await?,
        StatusCode::CREATED,
    ))
}

#[utoipa::path(
    get,
    path = "/api/exercise_instance/{exercise_instance_id}",
    params(
        ("exercise_instance_id" = Uuid, Path, description = "The ID of the exercise instance")
    ),
    security(
        ("access_token"= [])
    ),
    responses(
        (status = FOUND, description = "Exercise instance found", body = RouteSuccessExerciseInstance),
        (status = NOT_FOUND, description = "Exercise instance not found", body = RouteError),
        (status = UNAUTHORIZED, description = "Invalid authorization token", body = RouteError),
        (status = BAD_REQUEST, description = "Invalid ID format", body = RouteError),
    )
)]
pub async fn get_exercise_instance_by_id(
    user: User,
    State(pool): State<PgPool>,
    Path(exercise_instance_id): Path<Uuid>,
) -> RouteResponse<ExerciseInstance> {
    Ok(RouteSuccess::new(
        "Found exercise instance.",
        ExerciseInstance::from_id(user.id, exercise_instance_id, &pool).await?,
        StatusCode::FOUND,
    ))
}

#[utoipa::path(
    delete,
    path = "/api/exercise_instance/{exercise_instance_id}",
    params(
        ("exercise_instance_id" = Uuid, Path, description = "The ID of the exercise instance being deleted")
    ),
    security(
        ("access_token"= [])
    ),
    responses(
        (status = OK, description = "Exercise instance found and deleted", body = RouteSuccessUuid),
        (status = NOT_FOUND, description = "Exercise instance not found", body = RouteError),
        (status = UNAUTHORIZED, description = "Invalid authorization token", body = RouteError),
        (status = BAD_REQUEST, description = "Invalid ID format", body = RouteError),
    )
)]
pub async fn delete_exercise_instance_by_id(
    user: User,
    State(pool): State<PgPool>,
    Path(exercise_instance_id): Path<Uuid>,
) -> RouteResponse<Uuid> {
    Ok(RouteSuccess::new(
        "Exercise instance and sets related to it deleted.",
        ExerciseInstance::from_id(user.id, exercise_instance_id, &pool)
            .await?
            .delete(&pool)
            .await?,
        StatusCode::OK,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateExerciseInstanceCommentInput {
    #[validate(length(min = 1, max = 1000, message = "must be between 1 and 1000 characters"))]
    new_comment: String,
}

#[utoipa::path(
    post,
    path = "/api/exercise_instance/{exercise_instance_id}/comment",
    params(
        ("exercise_instance_id" = Uuid, Path, description = "The ID of the exercise instance being deleted")
    ),
    request_body = CreateExerciseInstanceCommentInput,
    security(
        ("access_token"= [])
    ),
    responses(
        (status = OK, description = "Comment added to exercise instance", body = RouteSuccessExerciseInstance),
        (status = NOT_FOUND, description = "Exercise instance not found", body = RouteError),
        (status = UNAUTHORIZED, description = "Invalid authorization token", body = RouteError),
        (status = BAD_REQUEST, description = "Invalid input for exercise instance", body = RouteError),
    )
)]
pub async fn add_exercise_instance_comment(
    user: User,
    State(pool): State<PgPool>,
    Path(exercise_instance_id): Path<Uuid>,
    Json(body): Json<CreateExerciseInstanceCommentInput>,
) -> RouteResponse<ExerciseInstance> {
    body.validate()?;

    let mut exercise_instance =
        ExerciseInstance::from_id(user.id, exercise_instance_id, &pool).await?;

    exercise_instance
        .add_comment(body.new_comment, &pool)
        .await?;

    Ok(RouteSuccess::new(
        "Appended a new comment.",
        exercise_instance,
        StatusCode::OK,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SetExerciseInstanceCommentInput {
    #[validate(length(min = 1, max = 1000, message = "must be between 1 and 1000 characters"))]
    comment: String,
}

#[utoipa::path(
    patch,
    path = "/api/exercise_instance/{exercise_instance_id}/comment/{comment_index}",
    params(
        ("exercise_instance_id" = Uuid, Path, description = "The ID of the exercise instance being edited"),
        ("comment_index" = i32, Path, description = "Index of the comment in exercise instance")
    ),
    request_body = SetExerciseInstanceCommentInput,
    security(
        ("access_token"= [])
    ),
    responses(
        (status = OK, description = "Comment with specified index edited in exercise instance", body = RouteSuccessExerciseInstance),
        (status = NOT_FOUND, description = "Exercise instance not found", body = RouteError),
        (status = UNAUTHORIZED, description = "Invalid authorization token", body = RouteError),
        (status = BAD_REQUEST, description = "Invalid input for edits or index", body = RouteError),
    )
)]
pub async fn set_exercise_instance_comment(
    user: User,
    State(pool): State<PgPool>,
    Path(path_args): Path<(Uuid, i32)>,
    Json(body): Json<SetExerciseInstanceCommentInput>,
) -> RouteResponse<ExerciseInstance> {
    body.validate()?;

    let mut exercise_instance = ExerciseInstance::from_id(user.id, path_args.0, &pool).await?;

    exercise_instance
        .set_comment(path_args.1, body.comment, &pool)
        .await?;

    Ok(RouteSuccess::new(
        "Comment updated.",
        exercise_instance,
        StatusCode::OK,
    ))
}

#[utoipa::path(
    delete,
    path = "/api/exercise_instance/{exercise_instance_id}/comment/{comment_index}",
    params(
        ("exercise_instance_id" = Uuid, Path, description = "The ID of the exercise instance being edited"),
        ("comment_index" = usize, Path, description = "Index of the comment in exercise instance")
    ),
    security(
        ("access_token"= [])
    ),
    responses(
        (status = OK, description = "Comment deleted", body = RouteSuccessUsize),
        (status = NOT_FOUND, description = "Exercise instance not found", body = RouteError),
        (status = UNAUTHORIZED, description = "Invalid authorization token", body = RouteError),
        (status = BAD_REQUEST, description = "Invalid index for deletion", body = RouteError),
    )
)]
pub async fn delete_exercise_instance_comment(
    user: User,
    State(pool): State<PgPool>,
    Path(path_args): Path<(Uuid, usize)>,
) -> RouteResponse<usize> {
    Ok(RouteSuccess::new(
        "Deleted comment.",
        ExerciseInstance::from_id(user.id, path_args.0, &pool)
            .await?
            .delete_comment(path_args.1, &pool)
            .await?,
        StatusCode::OK,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct EditExerciseInstanceInput {
    exercise_id: Option<Uuid>,
    // Could have more editable fields here if added
}

#[utoipa::path(
    patch,
    path = "/api/exercise_instance/{exercise_instance_id}",
    params(
        ("exercise_instance_id" = Uuid, Path, description = "The ID of the exercise instance being edited"),
    ),
    request_body = EditExerciseInstanceInput,
    security(
        ("access_token"= [])
    ),
    responses(
        (status = OK, description = "Changes made successfully", body = RouteSuccessExerciseInstance),
        (status = NOT_FOUND, description = "Exercise instance not found", body = RouteError),
        (status = UNAUTHORIZED, description = "Invalid authorization token", body = RouteError),
        (status = BAD_REQUEST, description = "Invalid input for edits", body = RouteError),
    )
)]
pub async fn edit_exercise_instance(
    user: User,
    State(pool): State<PgPool>,
    Path(exercise_instance_id): Path<Uuid>,
    Json(body): Json<EditExerciseInstanceInput>,
) -> RouteResponse<ExerciseInstance> {
    // Not for anything yet but maybe will have more fields in the future
    body.validate()?;

    let mut exercise_instance =
        ExerciseInstance::from_id(user.id, exercise_instance_id, &pool).await?;

    if let Some(id) = body.exercise_id {
        exercise_instance.set_exercise(id, &pool).await?;
    }

    Ok(RouteSuccess::new(
        "Requested changes made.",
        exercise_instance,
        StatusCode::OK,
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

        // Add 3 comments
        for i in 0..3 {
            server
                .post(&format!(
                    "/api/exercise_instance/{}/comment",
                    exercise_instance.id
                ))
                .json(&json!({"new_comment": format!("Comment {}", i)}))
                .await
                .assert_status_success();
        }

        // Delete comment at index 1 (middle one)
        server
            .delete(&format!(
                "/api/exercise_instance/{}/comment/1",
                exercise_instance.id
            ))
            .await
            .assert_status_success();

        // Edit comment at 0
        server
            .patch(&format!(
                "/api/exercise_instance/{}/comment/0",
                exercise_instance.id
            ))
            .json(&json!({"comment": "Comment"}))
            .await
            .assert_status_success();

        // Get comments
        let query = server
            .get(&format!("/api/exercise_instance/{}", exercise_instance.id))
            .await
            .json::<RouteSuccess<ExerciseInstance>>()
            .data;

        assert_eq!(query.comments.len(), 2);
        // 1 should be 2 now
        assert_eq!(query.comments.get(1).unwrap(), "Comment 2");
        // 0 edited
        assert_eq!(query.comments.get(0).unwrap(), "Comment");
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
}
