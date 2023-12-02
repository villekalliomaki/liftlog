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

    use crate::{api::response::RouteError, models::user::User, test_utils::database::test_user};

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
    async fn add_exercise_instance(pool: PgPool) {
        
    }

    #[sqlx::test]
    async fn delete_exercise_instance(pool: PgPool) {
        todo!();
    }

    #[sqlx::test]
    async fn reorder_exercise_instances(pool: PgPool) {
        todo!();
    }
}
