use uuid::Uuid;

// An ExerciseInstance has zero or more of these..
pub struct Set {
    // Primary key
    pub id: Uuid,
    // These are user specific
    pub user_id: Uuid,
    // An exercise instance is bound to an exercise instance
    pub exercise_instance_id: Uuid,
    // The weight used in kilograms, can be negative to signify an assisted lift
    pub weight: Option<f64>,
    // The reps are 0 or more
    pub reps: Option<usize>,
    // When created set is not completed and weight and reps have to be set before
    // marking it as complete
    pub completed: bool,
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use crate::models::{
        exercise::{Exercise, ExerciseKind},
        exercise_instance::ExerciseInstance,
        session::Session,
        user::User,
    };

    use super::*;

    async fn create_test_set(pool: &PgPool) -> (User, Exercise, Session, ExerciseInstance, Set) {
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

        let new_set: Set = Set::new(user.id, new_exercise_instance.id, &pool);

        (
            user,
            new_exercise,
            new_session,
            new_exercise_instance,
            new_set,
        )
    }

    #[sqlx::test]
    async fn create_and_query(pool: PgPool) {
        let (user, exercise, session, exercise_instance, set) = create_test_set(&pool).await;

        let set_query: Set = Set::from_id(user.id, set.id, &pool).await.unwrap();

        assert_eq!(set_query, set);

        // Queried by an instance, because they are included in it
        let exercise_instance_query: ExerciseInstance =
            ExerciseInstance::from_id(user.id, exercise_instance.id, &pool)
                .await
                .unwrap();

        assert_eq!(exercise_instance_query.sets.get(0), set);
        assert_eq!(exercise_instance_query.sets.len(), 1);
    }

    #[sqlx::test]
    async fn delete(pool: PgPool) {
        let (user, exercise, session, exercise_instance, mut set) = create_test_set(&pool).await;

        set.delete(&pool).await.unwrap();

        let set_query: Result<Set, RouteError> = Set::from_id(user.id, set.id, &pool).await;

        assert!(set_query.is_err());
    }

    #[sqlx::test]
    async fn save_weight(pool: PgPool) {
        todo!();
    }

    #[sqlx::test]
    async fn save_reps(pool: PgPool) {
        todo!();
    }

    #[sqlx::test]
    async fn mark_completed(pool: PgPool) {
        todo!();
    }

    #[sqlx::test]
    async fn mark_incomplete(pool: PgPool) {
        todo!();
    }
}