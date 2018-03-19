CREATE TABLE users (
    id              SERIAL PRIMARY KEY,
    email           TEXT NOT NULL,
    secret_hash     TEXT NOT NULL,
    username        TEXT NOT NULL,
    creation_date   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)