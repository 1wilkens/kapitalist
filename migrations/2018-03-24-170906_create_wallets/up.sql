CREATE TABLE wallets (
    id              BIGSERIAL NOT NULL PRIMARY KEY,
    user_id         BIGINT NOT NULL REFERENCES users(id),
    name            TEXT NOT NULL,
    wallet_type     TEXT NOT NULL,
    initial_balance BIGINT NOT NULL,
    current_balance BIGINT NOT NULL,
    color           TEXT NOT NULL,
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)
