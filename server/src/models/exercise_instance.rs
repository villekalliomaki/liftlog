use uuid::Uuid;

use super::exercise::Exercise;

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
    // The predefined exercise this one is an instance of
    pub exercise: Exercise,
    // Comments tied to this instance, and this way also the session
    pub comments: Vec<String>,
    // The sets included in the instance (order sensitive)
    pub sets: Vec<Set>,
}

// An ExerciseInstance has zero or more of these
pub struct Set {
    // Primary key
    pub id: Uuid,
    // These are user specific
    pub user_id: Uuid,
    // An exercise instance is bound to an exercise instance
    pub exercise_instance_id: Uuid,
    // The weight used in kilograms, can be negative to signify an assisted lift
    pub weight: isize,
    // The reps are 0 or more
    pub reps: usize,
}
