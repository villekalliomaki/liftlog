use uuid::Uuid;

// Reusable definition binding a name (and description) to a specific lift and a type
// and referenced in templates and sessions.
pub struct Exercise {
    // Primary key
    pub id: Uuid,
    // These are user specific
    pub user_id: Uuid,
    pub name: String,
    // Description is displayed in the details and could
    // define how the exercise is executed
    pub description: Option<String>,
    pub favourite: bool,
    // Notes visible in the sessions view, which are the same
    // between them and are only bound to the exercise
    pub notes: Vec<String>,
    pub kind: ExerciseKind,
}

// Used to categorize exercises and to define which way performance is measured in sets.
pub enum ExerciseKind {
    Dumbbell,
    Barbell,
    Cable,
    Machine,
    Bodyweight,
}
