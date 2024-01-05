use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal,
};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use tracing::{error, info, instrument};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::api::response::RouteError;

// An ExerciseInstance has zero or more of these..
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema, FromRow)]
pub struct Set {
    // Primary key
    pub id: Uuid,
    // These are user specific
    pub user_id: Uuid,
    // An exercise instance is bound to an exercise instance
    pub exercise_instance_id: Uuid,
    // The weight used in kilograms, can be negative to signify an assisted lift
    pub weight: Option<f32>,
    // The reps are 0 or more (checked by db)
    pub reps: Option<i32>,
    // When created set is not completed and weight and reps have to be set before
    // marking it as complete
    pub completed: bool,
    // Mainly for ordering of instances and can't be changed
    pub created: DateTime<Utc>,
}

impl Set {
    // Create a new set uncompleted withtout weight or reps set
    #[instrument]
    pub async fn new(
        user_id: Uuid,
        exercise_instance_id: Uuid,
        pool: &PgPool,
    ) -> Result<Self, RouteError> {
        info!("Creating new set");

        let exercise_instance_owner =
            sqlx::query("SELECT id FROM exercise_instances WHERE id = $1 AND user_id = $2")
                .bind(exercise_instance_id)
                .bind(user_id)
                .fetch_optional(pool)
                .await?;

        if exercise_instance_owner.is_none() {
            return Err(RouteError::new(
                "Invalid exercise instance ID",
                Some("session_id"),
                StatusCode::BAD_REQUEST,
            ));
        }

        Ok(sqlx::query_as!(
            Set,
            "INSERT INTO sets (user_id, exercise_instance_id) VALUES ($1, $2) RETURNING *;",
            user_id,
            exercise_instance_id
        )
        .fetch_one(pool)
        .await?)
    }

    // Get from ID and user
    #[instrument]
    pub async fn from_id(user_id: Uuid, set_id: Uuid, pool: &PgPool) -> Result<Self, RouteError> {
        info!("Querying a set based on ID");

        Ok(sqlx::query_as!(
            Set,
            "SELECT * FROM sets WHERE user_id = $1 AND id = $2",
            user_id,
            set_id
        )
        .fetch_one(pool)
        .await?)
    }

    // Delete set without modifying anythign else
    #[instrument]
    pub async fn delete(self, pool: &PgPool) -> Result<Uuid, RouteError> {
        info!("Deleting set (self)");

        sqlx::query!("DELETE FROM sets WHERE id = $1", self.id)
            .execute(pool)
            .await?;

        Ok(self.id)
    }

    #[instrument]
    pub async fn set_weight(
        &mut self,
        mut weight: Option<f32>,
        pool: &PgPool,
    ) -> Result<(), RouteError> {
        info!("Updating set weight");

        // All this just to round it to one decimal reliably...
        if let Some(not_rounded_weight) = weight {
            if let Some(decimal) = Decimal::from_f32(not_rounded_weight) {
                if let Some(rounded) = decimal.round_dp(1).to_f32() {
                    weight = Some(rounded);
                } else {
                    error!("Failed to round Decimal to f32: {}", decimal);

                    return Err(RouteError::new(
                        "Failed to process decimal number.",
                        Some("weight"),
                        StatusCode::BAD_REQUEST,
                    ));
                }
            } else {
                error!("Failed to convert f32 to Decimal: {}", not_rounded_weight);

                return Err(RouteError::new(
                    "Failed to process decimal number.",
                    Some("weight"),
                    StatusCode::BAD_REQUEST,
                ));
            }
        }

        self.weight = sqlx::query!(
            "UPDATE sets SET weight = $1 WHERE id = $2 RETURNING weight;",
            weight,
            self.id
        )
        .fetch_one(pool)
        .await?
        .weight;

        Ok(())
    }

    #[instrument]
    pub async fn set_reps(&mut self, reps: Option<i32>, pool: &PgPool) -> Result<(), RouteError> {
        info!("Updating set reps");

        self.reps = sqlx::query!(
            "UPDATE sets SET reps = $1 WHERE id = $2 RETURNING reps;",
            reps,
            self.id
        )
        .fetch_one(pool)
        .await?
        .reps;

        Ok(())
    }

    pub async fn set_complete(&mut self, pool: &PgPool) -> Result<(), RouteError> {
        self.set_completed_state(true, pool).await
    }

    pub async fn set_incomplete(&mut self, pool: &PgPool) -> Result<(), RouteError> {
        self.set_completed_state(false, pool).await
    }

    #[instrument]
    async fn set_completed_state(&mut self, state: bool, pool: &PgPool) -> Result<(), RouteError> {
        info!("Updating set completion state");

        self.completed = sqlx::query!(
            "UPDATE sets SET completed = $1 WHERE id = $2 RETURNING completed;",
            state,
            self.id
        )
        .fetch_one(pool)
        .await?
        .completed;

        Ok(())
    }
}

// Helper function to get all sets related to one exercise instance
//
// WARNING: User ownership of session IS NOT CHECKED
#[instrument]
pub async fn all_from_exercise_instance_id(
    user_id: Uuid,
    exercise_instance_id: Uuid,
    pool: &PgPool,
) -> Result<Vec<Set>, RouteError> {
    info!("Querying all sets of one exercise instance");

    Ok(sqlx::query_as(
        "SELECT * FROM sets WHERE user_id = $1 AND exercise_instance_id = $2 ORDER BY created",
    )
    .bind(user_id)
    .bind(exercise_instance_id)
    .fetch_all(pool)
    .await?)
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use crate::{
        models::{
            exercise::{Exercise, ExerciseKind},
            exercise_instance::ExerciseInstance,
            session::Session,
            user::User,
        },
        test_utils::api::create_test_user,
    };

    use super::*;

    async fn create_test_set(pool: &PgPool) -> (User, Exercise, Session, ExerciseInstance, Set) {
        let user = create_test_user(&pool).await;

        let new_exercise = Exercise::new(
            user.id,
            "Bench press",
            Some("description"),
            true,
            None::<&str>,
            ExerciseKind::Barbell,
            pool,
        )
        .await
        .unwrap();

        let new_session: Session =
            Session::new(user.id, "Test sessions", Some("Test description"), &pool)
                .await
                .unwrap();

        let new_exercise_instance: ExerciseInstance =
            ExerciseInstance::new(user.id, new_session.id, new_exercise.id, &pool)
                .await
                .unwrap();

        let new_set: Set = Set::new(user.id, new_exercise_instance.id, &pool)
            .await
            .unwrap();

        (
            user,
            new_exercise,
            new_session,
            new_exercise_instance,
            new_set,
        )
    }

    // Helper for querying sets via their instance
    async fn query_test_sets(
        user: &User,
        exercise_instance: &ExerciseInstance,
        pool: &PgPool,
    ) -> Vec<Set> {
        let query: ExerciseInstance =
            ExerciseInstance::from_id(user.id, exercise_instance.id, pool)
                .await
                .unwrap();

        query.sets
    }

    #[sqlx::test]
    async fn create_and_query(pool: PgPool) {
        let (user, _, _, exercise_instance, set) = create_test_set(&pool).await;

        let set_query: Set = Set::from_id(user.id, set.id, &pool).await.unwrap();

        assert_eq!(set_query, set);

        // Queried by an instance, because they are included in it
        let query_sets = query_test_sets(&user, &exercise_instance, &pool).await;

        assert_eq!(query_sets.get(0).unwrap().to_owned(), set);
        assert_eq!(query_sets.len(), 1);
    }

    #[sqlx::test]
    async fn delete(pool: PgPool) {
        let (user, _, _, _, set) = create_test_set(&pool).await;

        set.clone().delete(&pool).await.unwrap();

        let set_query: Result<Set, RouteError> = Set::from_id(user.id, set.id, &pool).await;

        assert!(set_query.is_err());
    }

    #[sqlx::test]
    async fn set_weight(pool: PgPool) {
        let (user, _, _, exercise_instance, mut set) = create_test_set(&pool).await;

        let weight: Option<f32> = Some(40.0);

        set.set_weight(weight, &pool).await.unwrap();

        assert_eq!(
            weight,
            query_test_sets(&user, &exercise_instance, &pool)
                .await
                .get(0)
                .unwrap()
                .weight
        );
    }

    #[sqlx::test]
    async fn set_reps(pool: PgPool) {
        let (user, _, _, exercise_instance, mut set) = create_test_set(&pool).await;

        let reps: Option<i32> = Some(14);

        set.set_reps(reps, &pool).await.unwrap();

        assert_eq!(
            reps,
            query_test_sets(&user, &exercise_instance, &pool)
                .await
                .get(0)
                .unwrap()
                .reps
        );
    }

    #[sqlx::test]
    async fn mark_completed(pool: PgPool) {
        let (user, _, _, exercise_instance, mut set) = create_test_set(&pool).await;

        set.set_complete(&pool).await.unwrap();

        assert!(
            query_test_sets(&user, &exercise_instance, &pool)
                .await
                .get(0)
                .unwrap()
                .completed
        );
    }

    #[sqlx::test]
    async fn mark_incomplete(pool: PgPool) {
        let (user, _, _, exercise_instance, mut set) = create_test_set(&pool).await;

        set.set_incomplete(&pool).await.unwrap();

        assert!(
            !query_test_sets(&user, &exercise_instance, &pool)
                .await
                .get(0)
                .unwrap()
                .completed
        );
    }
}
