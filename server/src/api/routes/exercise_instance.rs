#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    #[sqlx::test]
    async fn create_and_query(pool: PgPool) {
        todo!();
    }

    #[sqlx::test]
    async fn add_sets(pool: PgPool) {
        todo!();
    }

    // Try to change to an invalid exercise
    #[sqlx::test]
    async fn change_exercise(pool: PgPool) {
        todo!();
    }

    // Delete with sets linked to the instance
    #[sqlx::test]
    async fn delete_with_sets(pool: PgPool) {
        todo!();
    }

    // Try to delete an exercise which is used in an instance
    #[sqlx::test]
    async fn try_delete_used_exercise(pool: PgPool) {
        todo!();
    }

    #[sqlx::test]
    async fn get_all_for_session(pool: PgPool) {
        todo!();
    }
}
