CREATE TABLE IF NOT EXISTS sets (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id uuid NOT NULL,
    exercise_instance_id uuid NOT NULL,
    weight REAL,
    reps integer CHECK (reps >= 0),
    completed boolean NOT NULL DEFAULT false,
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT user_ownership FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT parent_exercise_instance FOREIGN KEY(exercise_instance_id) REFERENCES exercise_instances(id) ON DELETE CASCADE
);