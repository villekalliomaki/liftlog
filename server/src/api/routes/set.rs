#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    #[sqlx::test]
    async fn create_and_query(pool: PgPool) {
        todo!();
    }

    // Negative reps or not whole numbers
    #[sqlx::test]
    async fn set_invalid_reps(pool: PgPool) {
        todo!();
    }

    #[sqlx::test]
    async fn completion(pool: PgPool) {
        todo!();
    }

    // That that accuracy is one decimal, so 0.1kg
    #[sqlx::test]
    async fn set_weight(pool: PgPool) {
        todo!();
    }

    #[sqlx::test]
    async fn get_all_for_exercise_instance(pool: PgPool) {
        todo!();
    }
}
