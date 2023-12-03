use std::fmt::{Debug, Display};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::api::response::RouteError;

use super::exercise_instance::ExerciseInstance;

// A single session, can be in progess or finished.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema)]
pub struct Session {
    // Primary key
    pub id: Uuid,
    // These are user specific
    pub user_id: Uuid,
    // The template name, something generated or custom
    pub name: String,
    // Session specific comments or notes
    pub description: Option<String>,
    // Started, static
    pub started: DateTime<Utc>,
    // When it was finished
    pub finished: Option<DateTime<Utc>>,
    // Instances of predefined exercised, contains the kind, sets, reps, weight and more
    pub exercise_instances: Vec<ExerciseInstance>,
}

impl Session {
    // Create a new sessions wihtout any exercise instances,
    // set as started and not finished
    pub async fn new(
        user_id: Uuid,
        name: impl ToString + Display + Debug,
        description: Option<impl ToString + Display + Debug>,
        pool: &PgPool,
    ) -> Result<Self, RouteError> {
        todo!();
    }

    // Overwrites the name
    pub async fn set_name<S: ToString + Display + Debug>(
        &mut self,
        name: S,
        pool: &PgPool,
    ) -> Result<(), RouteError> {
        todo!();
    }

    // Overwrites the description
    pub async fn set_description<S: ToString + Display + Debug>(
        &mut self,
        description: Option<S>,
        pool: &PgPool,
    ) -> Result<(), RouteError> {
        todo!();
    }

    // Deleted the sessions and the exercise instances related and their sets related to it
    // Exercise is not deleted
    pub async fn delete(self, pool: &PgPool) -> Result<Uuid, RouteError> {
        todo!();
    }

    // Finish the session, permanent but doesn't lock exercise instances or their sets
    pub async fn mark_finished(&mut self, pool: &PgPool) -> Result<(), RouteError> {
        todo!();
    }

    // Get an instance from an ID, also fills the exercise instance field with it's
    // own field of sets
    pub async fn from_id(user_id: Uuid, id: Uuid, pool: &PgPool) -> Result<Self, RouteError> {
        todo!();
    }

    pub fn is_finished(&self) -> bool {
        self.finished.is_some()
    }

    // Swaps an exercise instance with the one before it, if one exists
    pub async fn move_exercise_instance_up(
        &mut self,
        moved_index: u64,
        pool: &PgPool,
    ) -> Result<(), RouteError> {
        todo!();
    }

    // Swaps an exercise instance with the one after it, if one exists
    pub async fn move_exercise_instance_down(
        &mut self,
        moved_index: u64,
        pool: &PgPool,
    ) -> Result<(), RouteError> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use crate::{
        api::response::RouteError,
        models::{
            exercise::{Exercise, ExerciseKind},
            user::User,
        },
        test_utils::database::test_user,
    };

    use super::*;

    async fn create_test_session(pool: &PgPool) -> (User, Session) {
        let user = test_user(&pool).await;

        let new_session: Session =
            Session::new(user.id, "Test sessions", Some("Test description"), &pool)
                .await
                .unwrap();

        (user, new_session)
    }

    #[sqlx::test]
    async fn create_and_query(pool: PgPool) {
        let (user, session) = create_test_session(&pool).await;

        let queried_new_session: Session =
            Session::from_id(user.id, session.id, &pool).await.unwrap();

        assert_eq!(session, queried_new_session);
    }

    #[sqlx::test]
    async fn edit_name_and_description(pool: PgPool) {
        let (user, mut session) = create_test_session(&pool).await;

        let new_name = "New test session";
        let new_description = None::<&str>;

        session.set_name(new_name, &pool).await.unwrap();
        session
            .set_description(new_description, &pool)
            .await
            .unwrap();

        let queried_session: Session = Session::from_id(user.id, session.id, &pool).await.unwrap();

        assert_eq!(queried_session.name, new_name);
        assert!(queried_session.description.is_none());
    }

    #[sqlx::test]
    async fn delete(pool: PgPool) {
        let (user, session) = create_test_session(&pool).await;

        let session_id = session.id.clone();

        session.delete(&pool).await.unwrap();

        let queried_session: Result<Session, RouteError> =
            Session::from_id(user.id, session_id, &pool).await;

        assert!(queried_session.is_err());
    }

    #[sqlx::test]
    async fn mark_finished(pool: PgPool) {
        let (user, mut session) = create_test_session(&pool).await;

        session.mark_finished(&pool).await.unwrap();

        let queried_session: Session = Session::from_id(user.id, session.id, &pool).await.unwrap();

        assert!(queried_session.is_finished());
        assert!(queried_session.finished.is_some());
    }

    #[sqlx::test]
    async fn reorder_exercise_instances(pool: PgPool) {
        let (user, session) = create_test_session(&pool).await;

        // Different exercises
        let new_exercises: Vec<Exercise> = vec![];
        for i in 0..2 {
            Exercise::new(
                user.id,
                format!("Bench press {}", i),
                Some("description"),
                true,
                None::<&str>,
                ExerciseKind::Barbell,
                &pool,
            )
            .await
            .unwrap();
        }

        // Create one instance for every exercise
        let mut exercise_instances: Vec<ExerciseInstance> = vec![];
        for i in new_exercises {
            let new_instance: ExerciseInstance =
                ExerciseInstance::new(user.id, session.id, i.id, &pool)
                    .await
                    .unwrap();

            exercise_instances.push(new_instance);
        }

        // Check the current order
        let mut queried_session_unchanged: Session =
            Session::from_id(user.id, session.id, &pool).await.unwrap();

        assert_eq!(
            queried_session_unchanged.exercise_instances,
            exercise_instances
        );

        // Change order,
        queried_session_unchanged
            .move_exercise_instance_up(1, &pool)
            .await
            .unwrap();

        // Moving up is swapping the index and the one before
        exercise_instances.swap(0, 1);
        let queried_session_changed_1: Session =
            Session::from_id(user.id, session.id, &pool).await.unwrap();

        assert_eq!(
            exercise_instances,
            queried_session_changed_1.exercise_instances
        );

        // Change order again
        queried_session_unchanged
            .move_exercise_instance_down(1, &pool)
            .await
            .unwrap();

        // Moving down is swapping the index and the one after
        exercise_instances.swap(1, 2);
        let queried_session_changed_2: Session =
            Session::from_id(user.id, session.id, &pool).await.unwrap();

        assert_eq!(
            exercise_instances,
            queried_session_changed_2.exercise_instances
        );
    }
}
