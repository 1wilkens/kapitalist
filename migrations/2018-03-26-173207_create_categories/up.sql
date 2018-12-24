CREATE TABLE categories (
    id          SERIAL NOT NULL PRIMARY KEY,
    parent_id   INTEGER REFERENCES categories(id),
    user_id     INTEGER REFERENCES users(id),
    name        TEXT NOT NULL,
    color       TEXT NOT NULL,
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Insert stock categories
INSERT INTO categories (parent_id, user_id, name, color)
VALUES (NULL, NULL, "Shopping", "0x0022DD");
