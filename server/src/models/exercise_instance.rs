use uuid::Uuid;

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
pub struct ExerciseInstance {
    // Primary key
    pub id: Uuid,
    // These are user specific
    pub user_id: Uuid,
    // An exercise instance is bound to a session
    pub session_id: Uuid,
    // Index number for displaying in a session
    pub session_index: usize,
    // The predefined exercise this one is an instance of
    pub exercise: Exercise,
    // Comments tied to this instance, and this way also the session
    pub comments: Vec<String>,
    // The sets included in the instance (order sensitive and immutable without deleting or adding)
    pub sets: Vec<Set>,
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use crate::{
        api::response::RouteError,
        models::{exercise::ExerciseKind, session::Session, user::User},
        test_utils::database::test_user,
    };

    use super::*;

    async fn create_test_exercise_instance(
        pool: &PgPool,
    ) -> (User, Exercise, Session, ExerciseInstance) {
        let user = test_user(&pool).await;

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

        (user, new_exercise, new_session, new_exercise_instance)
    }

    #[sqlx::test]
    async fn create_and_query(pool: PgPool) {
        let (user, exercise, session, exercise_instance) =
            create_test_exercise_instance(&pool).await;

        let exercise_instance_query: ExerciseInstance =
            ExerciseInstance::from_id(user.id, exercise_instance.id, &pool)
                .await
                .unwrap();

        assert_eq!(exercise_instance_query, exercise_instance);

        // Query exercise instances from a session
        let query: Session = Session::from_id(user.id, session.id, &pool).await.unwrap();

        assert_eq!(query.exercise_instances.get(0), exercise_instance);
        assert_eq!(query.exercise_instances.len(), 1);
    }

    #[sqlx::test]
    async fn delete(pool: PgPool) {
        let (user, exercise, session, mut exercise_instance) =
            create_test_exercise_instance(&pool).await;

        exercise_instance.delete(&pool).await.unwrap();

        let exercise_instance_query: Result<ExerciseInstance, RouteError> =
            ExerciseInstance::from_id(user.id, exercise_instance.id, &pool).await;

        assert!(exercise_instance_query.is_err());
    }

    #[sqlx::test]
    async fn edit_comments(pool: PgPool) {
        let (user, exercise, session, mut exercise_instance) =
            create_test_exercise_instance(&pool).await;

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

        let exercise_instance_query: Result<ExerciseInstance, RouteError> =
            ExerciseInstance::from_id(user.id, exercise_instance.id, &pool).await;

        assert_eq!(exercise_instance.comments.len(), 1);
        assert_eq!(
            exercise_instance.comments.get(0).unwrap(),
            &"Comment 3".to_string()
        );
    }

    #[sqlx::test]
    async fn change_exercise(pool: PgPool) {
        let (user, exercise, session, mut exercise_instance) =
            create_test_exercise_instance(&pool).await;

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

        assert_eq!(exercise_instance_query.exercise, new_exercise);
    }
}
