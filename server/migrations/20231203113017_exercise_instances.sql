CREATE TABLE IF NOT EXISTS exercise_instances (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id uuid NOT NULL,
    session_id uuid NOT NULL,
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    exercise_id uuid NOT NULL,
    comments TEXT [] NOT NULL DEFAULT array[]::text[],
    CONSTRAINT user_ownership FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT exercise_type FOREIGN KEY(exercise_id) REFERENCES exercises(id) ON DELETE RESTRICT,
    CONSTRAINT parent_session FOREIGN KEY(session_id) REFERENCES sessions(id) ON DELETE CASCADE
);