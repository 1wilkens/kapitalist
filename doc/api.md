# kapitalist - api

## v1

### user management / authentication
- POST /user  -- (register new user)
- GET /me -- (get own user details)
- PUT /me -- (update own user details)

- POST /auth  -- (obtain authentication token)

### wallets
- POST /wallet -- (create new wallet)
- GET /wallet/{id} -- (get wallet details)
- PUT /wallet/{id} -- (update wallet details)

### transactions
- POST /transaction -- (create new transaction)
- GET /transaction/{id} -- (get transaction details)
- PUT /transaction/{id} -- (update transaction details)
