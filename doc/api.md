# kapitalist - api

## v1

### user management / authentication
- GET /me -- (get own user details)
- PUT /me -- (update own user details)

- POST /user  -- (register new user)
- POST /auth  -- (obtain authentication token)

### wallets
- POST /wallets -- (create new wallet)
- GET /wallets -- (get own wallets)
- GET /wallets/{id} -- (get wallet details)
- PUT /wallets/{id} -- (update wallet details)

### transactions
- GET /transaction
- GET /