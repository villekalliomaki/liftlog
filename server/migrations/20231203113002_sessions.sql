CREATE TABLE IF NOT EXISTS sessions (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id uuid NOT NULL,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    started TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    finished TIMESTAMP WITH TIME ZONE,
    CONSTRAINT user_ownership FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
);