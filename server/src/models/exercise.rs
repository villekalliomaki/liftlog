use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::api::response::RouteError;

// Reusable definition binding a name (and description) to a specific lift and a type
// and referenced in templates and sessions.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema)]
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

// Used to categorize exercises and to define which way performance is measured in sets.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema)]
pub enum ExerciseKind {
    Dumbbell,
    Barbell,
    Cable,
    Machine,
    Bodyweight,
}

impl Exercise {
    // Creates a new exercise for the given user
    async fn new(
        user_id: Uuid,
        name: impl ToString + Display + Debug,
        description: Option<impl ToString + Display + Debug>,
        favourite: bool,
        notes: Option<impl ToString + Display + Debug>,
        kind: ExerciseKind,
        pool: &PgPool,
    ) -> Result<Exercise, RouteError> {
        todo!();
    }

    // Get an exercise from IDs
    async fn from_id(
        user_id: Uuid,
        exercise_id: Uuid,
        pool: &PgPool,
    ) -> Result<Exercise, RouteError> {
        todo!();
    }

    // Deletes self
    async fn delete(self, pool: &PgPool) -> Result<Uuid, RouteError> {
        todo!();
    }

    // Enable favourite state
    async fn enable_favourite(&mut self, pool: &PgPool) -> Result<(), RouteError> {
        todo!();
    }

    // Disable favourite state
    async fn disable_favourite(&mut self, pool: &PgPool) -> Result<(), RouteError> {
        todo!();
    }

    // Change all the text fields
    async fn change_fields(
        &mut self,
        new_name: impl ToString + Display + Debug,
        new_description: Option<impl ToString + Display + Debug>,
        new_notes: Option<impl ToString + Display + Debug>,
        pool: &PgPool,
    ) -> Result<(), RouteError> {
        todo!();
    }

    // Change all the text fields
    async fn set_kind(&mut self, new_kind: ExerciseKind, pool: &PgPool) -> Result<(), RouteError> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use crate::{models::user::User, test_utils::database::test_user};

    use super::*;

    // Test utility
    async fn create_user_and_exercise(pool: &PgPool) -> (User, Exercise) {
        let user = test_user(pool).await;

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
        let (user, new_exercise) = create_user_and_exercise(&pool).await;

        // Try to find it with the wrong user id
        let same_new_exercise_result =
            Exercise::from_id(Uuid::new_v4(), new_exercise.id, &pool).await;

        // Should not work
        assert!(same_new_exercise_result.is_err());
    }

    #[sqlx::test]
    async fn set_as_favouite(pool: PgPool) {
        let (user, mut new_exercise) = create_user_and_exercise(&pool).await;

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
        let new_desciption = None::<&str>;
        let new_notes = Some("something else");

        // Do the changes
        new_exercise
            .change_fields(new_name, new_desciption, new_notes, &pool)
            .await
            .unwrap();

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
