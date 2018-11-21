CREATE TABLE users (
    id         SERIAL NOT NULL PRIMARY KEY,
    email      TEXT NOT NULL,
    secret     TEXT NOT NULL,
    username   TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)
