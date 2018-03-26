CREATE TABLE categories (
    id              SERIAL NOT NULL PRIMARY KEY,
    name            TEXT NOT NULL,
    color           TEXT,
    created_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)