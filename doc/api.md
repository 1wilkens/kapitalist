# kapitalist - api

## v1

### User Management / Authentication
| Method | Endpoint | Payload/Params | Description |
| :--: | -- | -- | -- |
| POST | `/register` | UserCreationRequest | register new user |
| GET | `/me` | -- | get own user details |
| PUT | `/me` | UserUpdateRequest | update own user details |
|
| POST | `/auth` | TokenRequest | obtain authentication token |
|

### Wallets / transactions
| Method | Endpoint | Payload/Params | Description |
| :--: | -- | -- | -- |
| POST | `/wallet` | WalletCreationRequest | create new wallet |
| GET | `/wallet/{wid}` | `id` | get wallet details |
| PUT | `/wallet/{wid}` | WalletUpdateRequest | update wallet details |
|
| GET | `/wallet/{wid}/transactions` | `from, to` | get transaction history |
| POST | `/wallet/{wid}/transaction` | TransactionCreationRequest | create new transaction |
| GET | `/wallet/{wid}/transaction/{tid}` | -- | get transaction details |
| PUT | `/wallet/{wid}/transaction/{tid}` | TransactionUpdateRequest | update transaction details |
|

### Statistics
| Method | Endpoint | Payload/Params | Description |
| :--: | -- | -- | -- |
| GET | `/statistics` | `wallet_ids, from, to` | get general statistics for a list of wallets |
| GET | `/statistics/category` | `wallet_ids, from, to` | get category statistics for a list of wallets |
| GET | `/statistics/cashflow` | `wallet_ids, from, to` | get cashflow statistics for a list of wallets|
