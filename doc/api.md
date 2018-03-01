# kapitalist - api

## v1

### user management / authentication
- GET /me -- (get own user details)
- PUT /me -- (update own user details)

- POST /user  -- (register new user)
- POST /auth  -- (obtain authentication token)

### wallets
- POST /wallet -- (create new wallet)
- GET /wallets -- (get own wallets)
- GET /wallet/{id} -- (get wallet details)
- PUT /wallet/{id} -- (update wallet details)

### transactions
- GET /transaction
- GET /