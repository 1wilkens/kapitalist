CREATE TABLE categories (
    id          SERIAL NOT NULL PRIMARY KEY,
    user_id     INTEGER REFERENCES users(id),
    name        TEXT NOT NULL,
    color       TEXT,
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)
