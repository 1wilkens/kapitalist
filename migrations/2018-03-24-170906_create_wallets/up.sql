CREATE TABLE wallets (
    id              SERIAL NOT NULL PRIMARY KEY,
    user_id         INTEGER NOT NULL REFERENCES users(id),
    name            TEXT NOT NULL,
    initial_balance INTEGER NOT NULL,
    current_balance INTEGER NOT NULL,
    color           TEXT,
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)
