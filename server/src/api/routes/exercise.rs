use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Deserializer};
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    api::response::{RouteResponse, RouteSuccess},
    models::{
        exercise::{all_user_exercises, Exercise, ExerciseKind},
        user::User,
    },
};

// For serde...
fn default_as_false() -> bool {
    false
}

// Used for nested options: https://github.com/serde-rs/serde/issues/904
// Only way to have optional fields, which differentiate between the field being missing
// and being set to null js JSON
fn deserialize_optional_option<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Option::<T>::deserialize(deserializer).map(Some)
}

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct CreateExerciseInput {
    #[validate(length(min = 1, max = 30, message = "must be between 1 and 30 characters"))]
    name: String,
    #[validate(length(
        min = 1,
        max = 10000,
        message = "must be between 1 and 10000 characters"
    ))]
    description: Option<String>,
    #[serde(default = "default_as_false")]
    favourite: bool,
    #[validate(length(
        min = 1,
        max = 10000,
        message = "must be between 1 and 10000 characters"
    ))]
    notes: Option<String>,
    kind: ExerciseKind,
}

#[utoipa::path(
    post,
    path = "/api/exercise",
    request_body = CreateExerciseInput,
    responses(
        (status = CREATED, description = "New exercise created", body = RouteSuccessExercise),
        (status = UNAUTHORIZED, description = "Invalid authorization token", body = RouteError),
        (status = BAD_REQUEST, description = "Invalid input for exercise", body = RouteError),
    )
)]
pub async fn create_exercise(
    user: User,
    State(pool): State<PgPool>,
    Json(body): Json<CreateExerciseInput>,
) -> RouteResponse<Exercise> {
    body.validate()?;

    let new_exercise = Exercise::new(
        user.id,
        body.name,
        body.description,
        body.favourite,
        body.notes,
        body.kind,
        &pool,
    )
    .await?;

    Ok(RouteSuccess::new(
        format!("New exercise '{}' created.", &new_exercise.name),
        new_exercise,
        StatusCode::CREATED,
    ))
}

// All changes are optional, and if they do not exist in the input JSON
// they are not changed, meaning nulls can overwrite set values
#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct EditExerciseInput {
    #[validate(length(min = 1, max = 30, message = "must be between 1 and 30 characters"))]
    name: Option<String>,
    #[validate(length(
        min = 1,
        max = 10000,
        message = "must be between 1 and 10000 characters"
    ))]
    #[serde(default, deserialize_with = "deserialize_optional_option")]
    description: Option<Option<String>>,
    favourite: Option<bool>,
    #[validate(length(
        min = 1,
        max = 10000,
        message = "must be between 1 and 10000 characters"
    ))]
    #[serde(default, deserialize_with = "deserialize_optional_option")]
    notes: Option<Option<String>>,
    kind: Option<ExerciseKind>,
}

#[utoipa::path(
    patch,
    path = "/api/exercise/{exercise_id}",
    params(
        ("exercise_id" = Uuid, Path, description = "The ID of the edited exercise")
    ),
    request_body = EditExerciseInput,
    responses(
        (status = OK, description = "Exercise updated", body = RouteSuccessExercise),
        (status = NOT_FOUND, description = "Invalid exercise ID", body = RouteError),
        (status = UNAUTHORIZED, description = "Invalid authorization token", body = RouteError),
        (status = BAD_REQUEST, description = "Invalid input for exercise changes", body = RouteError),
        (status = BAD_REQUEST, description = "Invalid ID format", body = RouteError),
    )
)]
pub async fn edit_exercise(
    user: User,
    State(pool): State<PgPool>,
    Path(exercise_id): Path<Uuid>,
    Json(body): Json<EditExerciseInput>,
) -> RouteResponse<Exercise> {
    body.validate()?;

    let mut exercise = Exercise::from_id(user.id, exercise_id, &pool).await?;

    if let Some(new_name) = body.name {
        exercise.set_name(new_name, &pool).await?;
    }

    if let Some(new_description) = body.description {
        exercise.set_description(new_description, &pool).await?;
    }

    if let Some(new_favourite) = body.favourite {
        // This could just set it...
        if new_favourite {
            exercise.enable_favourite(&pool).await?;
        } else {
            exercise.disable_favourite(&pool).await?;
        }
    }

    if let Some(new_notes) = body.notes {
        exercise.set_notes(new_notes, &pool).await?;
    }

    if let Some(new_kind) = body.kind {
        exercise.set_kind(new_kind, &pool).await?;
    }

    Ok(RouteSuccess::new(
        "Updated fields.",
        exercise,
        StatusCode::OK,
    ))
}

#[utoipa::path(
    get,
    path = "/api/exercise/{exercise_id}",
    params(
        ("exercise_id" = Uuid, Path, description = "The ID of the exercise requested")
    ),
    responses(
        (status = OK, description = "Exercise found and returned", body = RouteSuccessExercise),
        (status = NOT_FOUND, description = "Invalid exercise ID", body = RouteError),
        (status = UNAUTHORIZED, description = "Invalid authorization token", body = RouteError),
        (status = BAD_REQUEST, description = "Invalid ID format", body = RouteError),
    )
)]
pub async fn get_exercise_by_id(
    user: User,
    State(pool): State<PgPool>,
    Path(exercise_id): Path<Uuid>,
) -> RouteResponse<Exercise> {
    Ok(RouteSuccess::new(
        "Exercise found.",
        Exercise::from_id(user.id, exercise_id, &pool).await?,
        StatusCode::OK,
    ))
}

#[utoipa::path(
    delete,
    path = "/api/exercise/{exercise_id}",
    params(
        ("exercise_id" = Uuid, Path, description = "The ID of the exercise requested")
    ),
    responses(
        (status = OK, description = "Exercise found and deleted", body = RouteSuccessUuid),
        (status = NOT_FOUND, description = "Invalid exercise ID", body = RouteError),
        (status = UNAUTHORIZED, description = "Invalid authorization token", body = RouteError),
        (status = BAD_REQUEST, description = "Invalid ID format", body = RouteError),
    )
)]
pub async fn delete_exercise_by_id(
    user: User,
    State(pool): State<PgPool>,
    Path(exercise_id): Path<Uuid>,
) -> RouteResponse<Uuid> {
    Ok(RouteSuccess::new(
        "Exercise deleted.",
        Exercise::from_id(user.id, exercise_id, &pool)
            .await?
            .delete(&pool)
            .await?,
        StatusCode::OK,
    ))
}

#[utoipa::path(
    get,
    path = "/api/exercise/all",
    responses(
        (status = OK, description = "Exercises returned", body = RouteSuccessExerciseVec),
        (status = UNAUTHORIZED, description = "Invalid authorization token", body = RouteError),
    )
)]
pub async fn get_user_exercises(
    user: User,
    State(pool): State<PgPool>,
) -> RouteResponse<Vec<Exercise>> {
    Ok(RouteSuccess::new(
        "Returned all exercises for your user.",
        all_user_exercises(user.id, None, &pool).await?,
        StatusCode::OK,
    ))
}

#[utoipa::path(
    get,
    path = "/api/exercise/all/{exercise_kind}",
    params(
        ("exercise_kind" = Uuid, Path, description = "The ID of the exercise requested")
    ),
    responses(
        (status = OK, description = "Exercises returned", body = RouteSuccessExerciseVec),
        (status = UNAUTHORIZED, description = "Invalid authorization token", body = RouteError),
        (status = BAD_REQUEST, description = "Invalid exercise kind format", body = RouteError),
    )
)]
pub async fn get_user_exercises_by_kind(
    user: User,
    State(pool): State<PgPool>,
    Path(exercise_kind): Path<ExerciseKind>,
) -> RouteResponse<Vec<Exercise>> {
    Ok(RouteSuccess::new(
        "Returned all exercises for your user of the specified kind.",
        all_user_exercises(user.id, Some(exercise_kind), &pool).await?,
        StatusCode::OK,
    ))
}

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
        assert_eq!(updated_test_exercise.favourite, test_exercise.favourite);
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

        for _ in 0..20 {
            create_test_exercise(&server, &header_name, &header_value).await;
        }

        // Note that this is /exercises not /exercise
        let users_exercises = server
            .get("/api/exercise/all")
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

        let users_exercises = server
            .get("/api/exercise/all/barbell")
            .add_header(header_name.clone(), header_value.clone())
            .await
            .json::<RouteSuccess<Vec<Exercise>>>()
            .data;

        // There should be just the one
        assert_eq!(users_exercises.len(), 1);
        assert_eq!(users_exercises.get(0).unwrap().id, barbell_exercise.id);
    }
}
