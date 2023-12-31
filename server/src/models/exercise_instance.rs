use std::fmt::{Debug, Display};

use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use tracing::{error, info, instrument};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{api::response::RouteError, models::set};

use super::{exercise::Exercise, set::Set};

// One instance of an existing exercise, containing:
// - Comments tied to the session
// - Sets
//   - Weight
//   - Reps
//   - Completed or not
//
// The sets are ordered, new ones are always added at the end and any of them
// can be deleted.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema, FromRow)]
pub struct ExerciseInstance {
    // Primary key
    pub id: Uuid,
    // These are user specific
    pub user_id: Uuid,
    // An exercise instance is bound to a session
    pub session_id: Uuid,
    // Mainly for ordering of instances and can't be changed
    pub created: DateTime<Utc>,
    // The predefined exercise this one is an instance of
    pub exercise_id: Uuid,
    // Comments tied to this instance, and this way also the session
    pub comments: Vec<String>,
    // The sets included in the instance (order sensitive and immutable without deleting or adding)
    #[sqlx(skip)]
    pub sets: Vec<Set>,
}

impl ExerciseInstance {
    // Create a new one with no comments or sets
    #[instrument]
    pub async fn new(
        user_id: Uuid,
        session_id: Uuid,
        exercise_id: Uuid,
        pool: &PgPool,
    ) -> Result<Self, RouteError> {
        // Make sure session and exercise are owned by the user
        let session_owner = sqlx::query("SELECT id FROM sessions WHERE id = $1 AND user_id = $2")
            .bind(session_id)
            .bind(user_id)
            .fetch_optional(pool)
            .await?;

        let exercise_owner = sqlx::query("SELECT id FROM exercises WHERE id = $1 AND user_id = $2")
            .bind(exercise_id)
            .bind(user_id)
            .fetch_optional(pool)
            .await?;

        if session_owner.is_none() {
            return Err(RouteError::new(
                "Invalid session ID",
                Some("session_id"),
                StatusCode::BAD_REQUEST,
            ));
        }

        if exercise_owner.is_none() {
            return Err(RouteError::new(
                "Invalid exercise ID",
                Some("exercise_id"),
                StatusCode::BAD_REQUEST,
            ));
        }

        Ok(sqlx::query_as("INSERT INTO exercise_instances (user_id, session_id, exercise_id) VALUES ($1, $2, $3) RETURNING *").bind(user_id).bind(session_id).bind(exercise_id).fetch_one(pool).await?)
    }

    // Get existing one by ID and user
    #[instrument]
    pub async fn from_id(
        user_id: Uuid,
        exercise_instance_id: Uuid,
        pool: &PgPool,
    ) -> Result<Self, RouteError> {
        let mut queried_instances: Self =
            sqlx::query_as("SELECT * FROM exercise_instances WHERE user_id = $1 AND id = $2")
                .bind(user_id)
                .bind(exercise_instance_id)
                .fetch_one(pool)
                .await?;

        queried_instances.sets =
            set::all_from_exercise_instance_id(user_id, exercise_instance_id, pool).await?;

        Ok(queried_instances)
    }

    // Delete the instance and the sets related to it
    #[instrument]
    pub async fn delete(self, pool: &PgPool) -> Result<Uuid, RouteError> {
        sqlx::query!("DELETE FROM exercise_instances WHERE id = $1", self.id)
            .execute(pool)
            .await?;

        Ok(self.id)
    }

    // Add a comment to the instance, always appended to the list
    #[instrument]
    pub async fn add_comment(
        &mut self,
        comment: impl ToString + Display + Debug,
        pool: &PgPool,
    ) -> Result<(), RouteError> {
        self.comments = sqlx::query!(
            "UPDATE exercise_instances SET comments = comments || $1 WHERE id = $2 RETURNING comments",
            &[comment.to_string()],
            self.id
        )
        .fetch_one(pool)
        .await?.comments;

        Ok(())
    }

    // Overwrite a comment of a specific index if it exists
    #[instrument]
    pub async fn set_comment(
        &mut self,
        index: i32,
        comment: impl ToString + Display + Debug,
        pool: &PgPool,
    ) -> Result<(), RouteError> {
        // Check that the index exists and return a clearer error if it doesn't
        if convert_index_type(index)? + 1 > self.comments.len() {
            Err(RouteError::new(
                format!("Comment doesn't exist at index {}.", index),
                Some("index"),
                StatusCode::NOT_FOUND,
            ))
        } else {
            self.comments = sqlx::query!(
                "UPDATE exercise_instances SET comments[$1] = $2 WHERE id = $3 RETURNING comments",
                index + 1,
                comment.to_string(),
                self.id
            )
            .fetch_one(pool)
            .await?
            .comments;

            Ok(())
        }
    }

    // Deleted a comment index if it exists
    #[instrument]
    pub async fn delete_comment(
        &mut self,
        index: usize,
        pool: &PgPool,
    ) -> Result<usize, RouteError> {
        match self.comments.get(index) {
            Some(value) => {
                self.comments = sqlx::query!(
                "UPDATE exercise_instances SET comments = array_remove(comments, $1) WHERE id = $2 RETURNING comments",
                value,
                self.id
            )
                .fetch_one(pool)
                .await?.comments;

                Ok(index)
            }
            None => Err(RouteError::new(
                "Tried to remove a comment in an exercise instance with invalid index.",
                None::<&str>,
                StatusCode::BAD_REQUEST,
            )),
        }
    }

    // Replaces the exercise with the given one if it exists
    #[instrument]
    pub async fn set_exercise(
        &mut self,
        exercise_id: Uuid,
        pool: &PgPool,
    ) -> Result<(), RouteError> {
        // Validate ownership of exercise
        let exercise = Exercise::from_id(self.user_id, exercise_id, pool).await?;

        sqlx::query!(
            "UPDATE exercise_instances SET exercise_id = $1 WHERE id = $2",
            exercise.id,
            self.id
        )
        .execute(pool)
        .await?;

        self.exercise_id = exercise_id;

        Ok(())
    }
}

// Helper to process input indexes
#[instrument]
fn convert_index_type(from: i32) -> Result<usize, RouteError> {
    // sqlx limtits the index to i32 and len is usize...
    match from.try_into() {
        Ok(value) => Ok(value),
        Err(error) => {
            error!("Failed to convert index from i32 to usize");

            Err(RouteError::new(
                format!("Failed internal index type conversion: {}", error),
                Some("index"),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

// Helper to get all exercise instances linked to an user and a session
// (also gets all sets with a helper function in the sets module)
//
// WARNING: User ownership of session IS NOT CHECKED
#[instrument]
pub async fn all_from_session_id(
    user_id: Uuid,
    session_id: Uuid,
    pool: &PgPool,
) -> Result<Vec<ExerciseInstance>, RouteError> {
    info!("Querying all exercise instances in one session");

    let mut all: Vec<ExerciseInstance> = sqlx::query_as(
        "SELECT * FROM exercise_instances WHERE user_id = $1 AND session_id = $2 ORDER BY created",
    )
    .bind(user_id)
    .bind(session_id)
    .fetch_all(pool)
    .await?;

    // Query all sets contained in each instance
    for instance in &mut all {
        instance.sets = set::all_from_exercise_instance_id(user_id, instance.id, pool).await?;
    }

    Ok(all)
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use crate::{
        api::response::RouteError,
        models::{
            exercise::{Exercise, ExerciseKind},
            session::Session,
        },
        test_utils::api::create_test_scenario,
    };

    use super::*;

    #[sqlx::test]
    async fn create_and_query(pool: PgPool) {
        let (_, user, _, _, session, exercise_instance, _) = create_test_scenario(&pool).await;

        let exercise_instance_query: ExerciseInstance =
            ExerciseInstance::from_id(user.id, exercise_instance.id, &pool)
                .await
                .unwrap();

        assert_eq!(exercise_instance_query, exercise_instance);

        // Query exercise instances from a session
        let query: Session = Session::from_id(user.id, session.id, &pool).await.unwrap();

        assert_eq!(query.exercise_instances.get(0).unwrap(), &exercise_instance);
        assert_eq!(query.exercise_instances.len(), 1);
    }

    #[sqlx::test]
    async fn delete(pool: PgPool) {
        let (_, user, _, _, _, exercise_instance, _) = create_test_scenario(&pool).await;

        exercise_instance.clone().delete(&pool).await.unwrap();

        let exercise_instance_query: Result<ExerciseInstance, RouteError> =
            ExerciseInstance::from_id(user.id, exercise_instance.id, &pool).await;

        assert!(exercise_instance_query.is_err());
    }

    #[sqlx::test]
    async fn edit_comments(pool: PgPool) {
        let (_, user, _, _, _, mut exercise_instance, _) = create_test_scenario(&pool).await;

        exercise_instance
            .add_comment("Comment 1", &pool)
            .await
            .unwrap();
        exercise_instance
            .add_comment("Comment 2", &pool)
            .await
            .unwrap();

        // Overwrite comment 2
        exercise_instance
            .set_comment(1, "Comment 3", &pool)
            .await
            .unwrap();

        // Delete comment 1
        exercise_instance.delete_comment(0, &pool).await.unwrap();

        let exercise_instance_query =
            ExerciseInstance::from_id(user.id, exercise_instance.id, &pool)
                .await
                .unwrap();

        assert_eq!(exercise_instance_query.comments.len(), 1);
        assert_eq!(
            exercise_instance_query.comments.get(0).unwrap(),
            &"Comment 3".to_string()
        );
    }

    #[sqlx::test]
    async fn change_exercise(pool: PgPool) {
        let (_, user, _, _, _, mut exercise_instance, _) = create_test_scenario(&pool).await;

        let new_exercise = Exercise::new(
            user.id,
            "Bench press",
            Some("description"),
            true,
            None::<&str>,
            ExerciseKind::Barbell,
            &pool,
        )
        .await
        .unwrap();

        exercise_instance
            .set_exercise(new_exercise.id, &pool)
            .await
            .unwrap();

        let exercise_instance_query: ExerciseInstance =
            ExerciseInstance::from_id(user.id, exercise_instance.id, &pool)
                .await
                .unwrap();

        assert_eq!(exercise_instance_query.exercise_id, new_exercise.id);
    }

    #[sqlx::test]
    async fn get_all_from_session_id(pool: PgPool) {
        let (_, user, _, exercise, session, _, _) = create_test_scenario(&pool).await;

        // Add some instances
        for _ in 1..10 {
            ExerciseInstance::new(user.id, session.id, exercise.id, &pool)
                .await
                .unwrap();
        }

        let instances = all_from_session_id(user.id, session.id, &pool)
            .await
            .unwrap();

        assert_eq!(instances.len(), 10);
    }
}
