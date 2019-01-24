CREATE TABLE transactions (
    id          BIGSERIAL NOT NULL PRIMARY KEY,
    wallet_id   BIGINT NOT NULL REFERENCES wallets(id),
    category_id BIGINT NOT NULL REFERENCES categories(id),
    amount      BIGINT NOT NULL,
    ts          TIMESTAMP NOT NULL
)
