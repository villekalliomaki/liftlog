// A single session which can be empty of filled based on a template.
// Can be in progess or finished.
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
    // Session exercises
    pub session_exercises: Vec,
}
