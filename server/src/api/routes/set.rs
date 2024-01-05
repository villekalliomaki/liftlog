use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    api::{
        extractors::json::ValidatedJson,
        response::{RouteResponse, RouteSuccess},
    },
    models::{set::Set, user::User},
};

use super::deserialize_optional_option;

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct CreateSetInput {
    exercise_instance_id: Uuid,
}

#[utoipa::path(
    post,
    path = "/api/set",
    request_body = CreateSetInput,
    security(
        ("access_token"= [])
    ),
    responses(
        (status = CREATED, description = "New set created", body = RouteSuccessSet),
        (status = UNAUTHORIZED, description = "Invalid authorization token", body = RouteError),
        (status = NOT_FOUND, description = "Exercise instance not found", body = RouteError),
        (status = BAD_REQUEST, description = "Invalid input for set", body = RouteError),
    )
)]
pub async fn create_set(
    user: User,
    State(pool): State<PgPool>,
    ValidatedJson(body): ValidatedJson<CreateSetInput>,
) -> RouteResponse<Set> {
    Ok(RouteSuccess::new(
        "New set created without reps or weight.",
        Set::new(user.id, body.exercise_instance_id, &pool).await?,
        StatusCode::CREATED,
    ))
}

#[utoipa::path(
    get,
    path = "/api/set/{set_id}",
    params(
        ("set_id" = Uuid, Path, description = "The ID of the set")
    ),
    security(
        ("access_token"= [])
    ),
    responses(
        (status = OK, description = "Set found", body = RouteSuccessSet),
        (status = NOT_FOUND, description = "Set not found", body = RouteError),
        (status = UNAUTHORIZED, description = "Invalid authorization token", body = RouteError),
        (status = BAD_REQUEST, description = "Invalid ID format", body = RouteError),
    )
)]
pub async fn get_set_by_id(
    user: User,
    State(pool): State<PgPool>,
    Path(set_id): Path<Uuid>,
) -> RouteResponse<Set> {
    Ok(RouteSuccess::new(
        "Found set from ID.",
        Set::from_id(user.id, set_id, &pool).await?,
        StatusCode::OK,
    ))
}

#[utoipa::path(
    delete,
    path = "/api/set/{set_id}",
    params(
        ("set_id" = Uuid, Path, description = "The ID of the set")
    ),
    security(
        ("access_token"= [])
    ),
    responses(
        (status = OK, description = "Set deleted", body = RouteSuccessUuid),
        (status = NOT_FOUND, description = "Set not found", body = RouteError),
        (status = UNAUTHORIZED, description = "Invalid authorization token", body = RouteError),
        (status = BAD_REQUEST, description = "Invalid ID format", body = RouteError),
    )
)]
pub async fn delete_set(
    user: User,
    State(pool): State<PgPool>,
    Path(set_id): Path<Uuid>,
) -> RouteResponse<Uuid> {
    Ok(RouteSuccess::new(
        "Deleted specified set.",
        Set::from_id(user.id, set_id, &pool)
            .await?
            .delete(&pool)
            .await?,
        StatusCode::OK,
    ))
}

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct EditSetInput {
    #[serde(default, deserialize_with = "deserialize_optional_option")]
    weight: Option<Option<f32>>,
    #[serde(default, deserialize_with = "deserialize_optional_option")]
    reps: Option<Option<i32>>,
    completed: Option<bool>,
}

#[utoipa::path(
    patch,
    path = "/api/set/{set_id}",
    params(
        ("set_id" = Uuid, Path, description = "The ID of the exercise instance being edited"),
    ),
    request_body = EditSetInput,
    security(
        ("access_token"= [])
    ),
    responses(
        (status = OK, description = "Requseted changes made successfully", body = RouteSuccessSet),
        (status = NOT_FOUND, description = "Set not found", body = RouteError),
        (status = UNAUTHORIZED, description = "Invalid authorization token", body = RouteError),
        (status = BAD_REQUEST, description = "Invalid input for changes", body = RouteError),
    )
)]
pub async fn edit_set(
    user: User,
    State(pool): State<PgPool>,
    Path(set_id): Path<Uuid>,
    ValidatedJson(body): ValidatedJson<EditSetInput>,
) -> RouteResponse<Set> {
    let mut set = Set::from_id(user.id, set_id, &pool).await?;

    if let Some(weight) = body.weight {
        set.set_weight(weight, &pool).await?
    }

    if let Some(reps) = body.reps {
        set.set_reps(reps, &pool).await?
    }

    if let Some(completed) = body.completed {
        if completed {
            set.set_complete(&pool).await?;
        } else {
            set.set_incomplete(&pool).await?;
        }
    }

    Ok(RouteSuccess::new(
        "Requseted changes to set made successfully.",
        set,
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
        models::{exercise_instance::ExerciseInstance, set::Set},
        test_utils::api::{create_test_exercise_instance, create_test_scenario, create_test_set},
    };

    // Queries set over API and panics on failure
    async fn query_set(server: &TestServer, set_id: Uuid) -> Set {
        server
            .get(&format!("/api/set/{}", set_id))
            .await
            .json::<RouteSuccess<Set>>()
            .data
    }

    #[sqlx::test]
    async fn create_and_query(pool: PgPool) {
        let (server, _, _, _, _, _, set) = create_test_scenario(&pool).await;

        let valid_query = query_set(&server, set.id).await;

        assert_eq!(valid_query, set);

        let invalid_query = server.get(&format!("/api/set/{}", Uuid::new_v4())).await;

        invalid_query.assert_status_failure();
    }

    #[sqlx::test]
    async fn delete_set(pool: PgPool) {
        let (server, _, _, _, _, _, set) = create_test_scenario(&pool).await;

        server
            .delete(&format!("/api/set/{}", set.id))
            .await
            .assert_status_success();

        server
            .get(&format!("/api/set/{}", set.id))
            .await
            .assert_status_failure();
    }

    // Negative reps or not whole numbers
    #[sqlx::test]
    async fn set_invalid_reps(pool: PgPool) {
        let (server, _, _, _, _, _, set) = create_test_scenario(&pool).await;

        // Negative reps
        server
            .patch(&format!("/api/set/{}", set.id))
            .json(&json!({"reps": -1}))
            .await
            .assert_status_failure();

        // Float number reps
        server
            .patch(&format!("/api/set/{}", set.id))
            .json(&json!({"reps": 10.5}))
            .await
            .assert_status_failure();
    }

    #[sqlx::test]
    async fn completion(pool: PgPool) {
        let (server, _, _, _, _, _, set) = create_test_scenario(&pool).await;

        server
            .patch(&format!("/api/set/{}", set.id))
            .json(&json!({"completed": true}))
            .await
            .assert_status_success();

        assert!(query_set(&server, set.id).await.completed);
    }

    // That that accuracy is one decimal, so 0.1kg
    #[sqlx::test]
    async fn set_weight(pool: PgPool) {
        let (server, _, _, _, _, _, set) = create_test_scenario(&pool).await;

        // Whole number should just work and be converted to a float
        let query_whole_number = server
            .patch(&format!("/api/set/{}", set.id))
            .json(&json!({"weight": 50}))
            .await
            .json::<RouteSuccess<Set>>()
            .data;

        assert_eq!(query_whole_number.weight.unwrap(), 50.0);

        // Should be accurate to only 0.1kg
        let query_accuracy = server
            .patch(&format!("/api/set/{}", set.id))
            .json(&json!({"weight": 10.08}))
            .await
            .json::<RouteSuccess<Set>>()
            .data;

        assert_eq!(query_accuracy.weight.unwrap(), 10.1);

        // Negative should also work for assisted lifts
        let query_negative = server
            .patch(&format!("/api/set/{}", set.id))
            .json(&json!({"weight": -10.0001}))
            .await
            .json::<RouteSuccess<Set>>()
            .data;

        assert_eq!(query_negative.weight.unwrap(), -10.0);
    }

    #[sqlx::test]
    async fn get_all_for_exercise_instance(pool: PgPool) {
        let (server, _, _, exercise, session, exercise_instance, _) =
            create_test_scenario(&pool).await;

        // Create a few more
        for _ in 0..3 {
            create_test_set(&server, exercise_instance.id).await;
        }

        // Also a few for another exercise instance
        let another_instance =
            create_test_exercise_instance(&server, session.id, exercise.id).await;

        for _ in 0..10 {
            create_test_set(&server, another_instance.id).await;
        }

        let query_all = server
            .get(&format!("/api/exercise_instance/{}", exercise_instance.id))
            .await
            .json::<RouteSuccess<ExerciseInstance>>()
            .data;

        assert_eq!(query_all.sets.len(), 4);
    }
}
