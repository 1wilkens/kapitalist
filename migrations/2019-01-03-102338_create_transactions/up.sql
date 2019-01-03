CREATE TABLE transactions (
    id                      SERIAL NOT NULL PRIMARY KEY,
    source_wallet_id        INTEGER NOT NULL REFERENCES wallets(id),
    destination_wallet_id   INTEGER REFERENCES wallets(id),
    category_id             INTEGER NOT NULL REFERENCES categories(id),
    amount                  INTEGER NOT NULL,
    ts                      TIMESTAMP NOT NULL
)
