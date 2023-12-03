use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::exercise_instance::ExerciseInstance;

// A single session, can be in progess or finished.
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
        assert_eq!(queried_session.description, new_description);
    }

    #[sqlx::test]
    async fn delete(pool: PgPool) {
        let (user, mut session) = create_test_session(&pool).await;

        session.delete(&pool).await.unwrap();

        let queried_session: Result<Session, RouteError> =
            Session::from_id(user.id, session.id, &pool).await;

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
        let (user, mut session) = create_test_session(&pool).await;

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
        let mut queried_session_changed_1: Session =
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
        let mut queried_session_changed_2: Session =
            Session::from_id(user.id, session.id, &pool).await.unwrap();

        assert_eq!(
            exercise_instances,
            queried_session_changed_2.exercise_instances
        );
    }
}
