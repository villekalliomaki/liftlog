CREATE TYPE exercise_kind AS ENUM (
    'DUMBBELL',
    'BARBELL',
    'CABLE',
    'MACHINE',
    'BODYWEIGHT'
);
CREATE TABLE IF NOT EXISTS exercises (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id uuid NOT NULL,
    name VARCHAR(30) NOT NULL,
    description TEXT,
    favourite boolean NOT NULL DEFAULT false,
    notes TEXT,
    kind exercise_kind NOT NULL,
    CONSTRAINT user_ownership FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
);