use uuid::Uuid;


// Used to create a new session with predefined exercises and sets (reps, time and/or weights are taken from the last session).
pub struct Template {
    // Primary key
    pub id: Uuid,
    // These are user specific
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub favourite: bool,
    // ???
}