use serde::{Deserialize, Serialize};
use sqlx::{pool, prelude::FromRow, PgPool};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::api::response::RouteError;

use super::exercise_instance;

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
    pub weight: Option<f64>,
    // The reps are 0 or more
    pub reps: Option<u64>,
    // When created set is not completed and weight and reps have to be set before
    // marking it as complete
    pub completed: bool,
}

impl Set {
    // Create a new set uncompleted withtout weight or reps set
    pub async fn new(
        user_id: Uuid,
        exercise_instance_id: Uuid,
        pool: &PgPool,
    ) -> Result<Self, RouteError> {
        todo!();
    }

    // Get from ID and user
    pub async fn from_id(user_id: Uuid, set_id: Uuid, pool: &PgPool) -> Result<Self, RouteError> {
        todo!();
    }

    // Delete set without modifying anythign else
    pub async fn delete(self, pool: &PgPool) -> Result<Uuid, RouteError> {
        todo!();
    }

    pub async fn set_weight(
        &mut self,
        weight: Option<f64>,
        pool: &PgPool,
    ) -> Result<(), RouteError> {
        todo!();
    }

    pub async fn set_reps(&mut self, reps: Option<u64>, pool: &PgPool) -> Result<(), RouteError> {
        todo!();
    }

    pub async fn set_complete(&mut self, pool: &PgPool) -> Result<(), RouteError> {
        todo!();
    }

    pub async fn set_incomplete(&mut self, pool: &PgPool) -> Result<(), RouteError> {
        todo!();
    }
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
        test_utils::database::test_user,
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

        let weight: Option<f64> = Some(40.0);

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

        let reps: Option<u64> = Some(14);

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
