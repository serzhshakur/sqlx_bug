CREATE TABLE users (
    id TEXT PRIMARY KEY NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT,
    username TEXT,
    UNIQUE (id)
);
