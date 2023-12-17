use std::fmt::{Debug, Display};

use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use tracing::{info, instrument};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::api::response::RouteError;

// Reusable definition binding a name (and description) to a specific lift and a type
// and referenced in templates and sessions.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema, FromRow)]
pub struct Exercise {
    // Primary key
    pub id: Uuid,
    // These are user specific
    pub user_id: Uuid,
    // Name of the movement, which should not include the kind
    pub name: String,
    // Description is displayed in the details and could
    // define how the exercise is executed
    pub description: Option<String>,
    // Is displayed at the top
    pub favourite: bool,
    // Notes visible in the sessions view, which are the same
    // between them and are only bound to the exercise
    pub notes: Option<String>,
    // Kind is just visual and any additional weight is not calculated at the set level
    pub kind: ExerciseKind,
}

// Used to categorize exercises
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema, sqlx::Type)]
#[sqlx(type_name = "exercise_kind", rename_all = "UPPERCASE")]
#[serde(rename_all = "lowercase")]
pub enum ExerciseKind {
    Dumbbell,
    Barbell,
    Cable,
    Machine,
    Bodyweight,
}

impl Exercise {
    // Creates a new exercise for the given user
    #[instrument]
    pub async fn new(
        user_id: Uuid,
        name: impl ToString + Display + Debug,
        description: Option<impl ToString + Display + Debug>,
        favourite: bool,
        notes: Option<impl ToString + Display + Debug>,
        kind: ExerciseKind,
        pool: &PgPool,
    ) -> Result<Exercise, RouteError> {
        info!("Creating a new exercise '{}' for user '{}'", name, user_id);

        Ok(sqlx::query_as!(
            Exercise,
            r#"
            INSERT INTO exercises (user_id, name, description, favourite, notes, kind)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, user_id, name, description, favourite, notes, kind AS "kind: ExerciseKind"
            "#,
            user_id,
            name.to_string(),
            description.map_or(None, |i| Some(i.to_string())),
            favourite,
            notes.map_or(None, |i| Some(i.to_string())),
            kind as _
        )
        .fetch_one(pool)
        .await?)
    }

    // Get an exercise from IDs
    #[instrument]
    pub async fn from_id(
        user_id: Uuid,
        exercise_id: Uuid,
        pool: &PgPool,
    ) -> Result<Exercise, RouteError> {
        info!("Querying exercise {} of user {}", exercise_id, user_id);

        Ok(sqlx::query_as!(
            Exercise,
            r#"
            SELECT id, user_id, name, description, favourite, notes, kind AS "kind: ExerciseKind"
            FROM exercises WHERE user_id = $1 AND id = $2 LIMIT 1
            "#,
            user_id,
            exercise_id
        )
        .fetch_one(pool)
        .await?)
    }

    // Deletes self
    #[instrument]
    pub async fn delete(self, pool: &PgPool) -> Result<Uuid, RouteError> {
        info!("Deleting exercise {}", self.id);

        let res = sqlx::query!(
            "DELETE FROM exercises WHERE user_id = $1 AND id = $2",
            self.user_id,
            self.id
        )
        .execute(pool)
        .await?;

        if res.rows_affected() == 1 {
            Ok(self.id)
        } else {
            Err(RouteError::new(
                "Exercise not fould, so nothing was deleted.",
                None::<&str>,
                StatusCode::NOT_FOUND,
            ))
        }
    }

    // Enable favourite state
    #[instrument]
    pub async fn enable_favourite(&mut self, pool: &PgPool) -> Result<(), RouteError> {
        self.set_favourite_state(true, pool).await
    }

    // Disable favourite state
    #[instrument]
    pub async fn disable_favourite(&mut self, pool: &PgPool) -> Result<(), RouteError> {
        self.set_favourite_state(false, pool).await
    }

    // Helped to toggle favourite state
    #[instrument]
    async fn set_favourite_state(
        &mut self,
        favorite: bool,
        pool: &PgPool,
    ) -> Result<(), RouteError> {
        info!("Setting favourite state of exercise");

        let favourite_updated = sqlx::query!(
            "UPDATE exercises SET favourite = $1 WHERE id = $2 AND user_id = $3 RETURNING favourite",
            favorite,
            self.id,
            self.user_id
        ).fetch_one(pool).await?;

        self.favourite = favourite_updated.favourite;

        Ok(())
    }

    // Change what kind of exercise this is
    #[instrument]
    pub async fn set_kind(
        &mut self,
        new_kind: ExerciseKind,
        pool: &PgPool,
    ) -> Result<(), RouteError> {
        info!("Setting exercise kind");

        self.kind = sqlx::query!(
            r#"UPDATE exercises SET kind = $1 WHERE id = $2 AND user_id = $3 RETURNING kind AS "kind: ExerciseKind""#,
            new_kind as _,
            self.id,
            self.user_id
        ).fetch_one(pool).await?.kind;

        Ok(())
    }

    // Change the name, overwrites to null if set to None
    #[instrument]
    pub async fn set_name(
        &mut self,
        new_name: impl ToString + Display + Debug,
        pool: &PgPool,
    ) -> Result<(), RouteError> {
        info!("Setting exercise name");

        self.name = sqlx::query!(
            "UPDATE exercises SET name = $1 WHERE id = $2 AND user_id = $3 RETURNING name",
            new_name.to_string(),
            self.id,
            self.user_id
        )
        .fetch_one(pool)
        .await?
        .name;

        Ok(())
    }

    // Change the description, overwrites to null if set to None
    #[instrument]
    pub async fn set_description(
        &mut self,
        new_description: Option<impl ToString + Display + Debug>,
        pool: &PgPool,
    ) -> Result<(), RouteError> {
        info!("Updating exercise description");

        self.description = sqlx::query!(
            "UPDATE exercises SET description = $1 WHERE id = $2 AND user_id = $3 RETURNING description",
            new_description.map_or(None, |i| Some(i.to_string())),
            self.id,
            self.user_id
        )
        .fetch_one(pool)
        .await?
        .description;

        Ok(())
    }

    // Change the notes, overwrites to null if set to None
    #[instrument]
    pub async fn set_notes(
        &mut self,
        new_notes: Option<impl ToString + Display + Debug>,
        pool: &PgPool,
    ) -> Result<(), RouteError> {
        info!("Updating exercise notes");

        self.notes = sqlx::query!(
            "UPDATE exercises SET notes = $1 WHERE id = $2 AND user_id = $3 RETURNING notes",
            new_notes.map_or(None, |i| Some(i.to_string())),
            self.id,
            self.user_id
        )
        .fetch_one(pool)
        .await?
        .notes;

        Ok(())
    }
}

// Get all the users exercises, if Kind is specified it's included as a filter
// Not tested here, because this helper is covered by the route tests
#[instrument]
pub async fn all_user_exercises(
    user_id: Uuid,
    optional_kind: Option<ExerciseKind>,
    pool: &PgPool,
) -> Result<Vec<Exercise>, RouteError> {
    info!("Querying all exercises of an user");

    match optional_kind {
        Some(kind) => Ok(sqlx::query_as!(
            Exercise,
            r#"SELECT id, user_id, name, description, favourite, notes, kind AS "kind: ExerciseKind" FROM exercises WHERE user_id = $1 AND kind = $2"#,
            user_id,
            kind as _
        )
        .fetch_all(pool)
        .await?),
        None => Ok(sqlx::query_as!(
            Exercise,
            r#"SELECT id, user_id, name, description, favourite, notes, kind AS "kind: ExerciseKind" FROM exercises WHERE user_id = $1"#,
            user_id
        )
        .fetch_all(pool).await?)
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use crate::{models::user::User, test_utils::database::create_test_user};

    use super::*;

    // Test utility
    async fn create_user_and_exercise(pool: &PgPool) -> (User, Exercise) {
        let user = create_test_user(pool).await;

        // Just create a new one
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

        (user, new_exercise)
    }

    #[sqlx::test]
    async fn create(pool: PgPool) {
        let (user, new_exercise) = create_user_and_exercise(&pool).await;

        // Should be queryable
        let same_new_exercise: Exercise = Exercise::from_id(user.id, new_exercise.id, &pool)
            .await
            .unwrap();

        // Should be the same
        assert_eq!(new_exercise, same_new_exercise);
    }

    #[sqlx::test]
    async fn delete(pool: PgPool) {
        let (user, new_exercise) = create_user_and_exercise(&pool).await;

        let new_exercise_id = new_exercise.id;

        // Delete it
        new_exercise.delete(&pool).await.unwrap();

        // Should not exist
        let same_new_exercise_result = Exercise::from_id(user.id, new_exercise_id, &pool).await;

        assert!(same_new_exercise_result.is_err());
    }

    #[sqlx::test]
    async fn user_ownership(pool: PgPool) {
        let (_, new_exercise) = create_user_and_exercise(&pool).await;

        // Try to find it with the wrong user id
        let same_new_exercise_result =
            Exercise::from_id(Uuid::new_v4(), new_exercise.id, &pool).await;

        // Should not work
        assert!(same_new_exercise_result.is_err());
    }

    #[sqlx::test]
    async fn set_as_favouite(pool: PgPool) {
        let (_, mut new_exercise) = create_user_and_exercise(&pool).await;

        // Enable
        new_exercise.enable_favourite(&pool).await.unwrap();

        assert!(new_exercise.favourite);

        // Disable
        new_exercise.disable_favourite(&pool).await.unwrap();

        assert!(!new_exercise.favourite);
    }

    #[sqlx::test]
    async fn change_text_fields(pool: PgPool) {
        let (user, mut new_exercise) = create_user_and_exercise(&pool).await;

        let new_name = "Squat";
        let new_description = None::<&str>;
        let new_notes = Some("something else");

        // Do the changes
        new_exercise.set_name(new_name, &pool).await.unwrap();
        new_exercise
            .set_description(new_description, &pool)
            .await
            .unwrap();
        new_exercise.set_notes(new_notes, &pool).await.unwrap();

        // Get the same exercise from db
        let same_new_exercise: Exercise = Exercise::from_id(user.id, new_exercise.id, &pool)
            .await
            .unwrap();

        // Validate changes
        assert_eq!(new_exercise.name, same_new_exercise.name);
        assert_eq!(new_exercise.description, same_new_exercise.description);
        assert_eq!(new_exercise.notes, same_new_exercise.notes);
    }

    #[sqlx::test]
    async fn change_kind(pool: PgPool) {
        let (user, mut new_exercise) = create_user_and_exercise(&pool).await;

        new_exercise
            .set_kind(ExerciseKind::Cable, &pool)
            .await
            .unwrap();

        // Get the same exercise from db
        let same_new_exercise: Exercise = Exercise::from_id(user.id, new_exercise.id, &pool)
            .await
            .unwrap();

        assert_eq!(same_new_exercise.kind, ExerciseKind::Cable);
    }
}
