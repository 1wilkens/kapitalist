# kapitalist - api

## v1

### User Management / Authentication

| Method | Endpoint | Payload/Params | Result | Description |
| :--: | -- | -- | -- | -- |
| POST | `/register` | UserCreationRequest | User | register new user |
| GET | `/me` | - | User | get own user details |
| PUT | `/me` | UserUpdateRequest | User | update own user details |
| POST | `/token` | TokenRequest | TokenResponse | obtain authentication token |

### Wallets

| Method | Endpoint | Payload/Params | Result | Description |
| :--: | -- | -- | -- | -- |
| POST | `/wallet` | WalletCreationRequest | Wallet | create new wallet |
| GET | `/wallet/{wid}` | - | Wallet | get wallet details |
| PUT | `/wallet/{wid}` | WalletUpdateRequest | Wallet | update wallet details |
| DELETE | `/wallet/{wid}` | - | HTTP 200? | delete wallet |

### Categories

| Method | Endpoint | Payload/Params | Result | Description |
| :--: | -- | -- | -- | -- |
| POST | `/category` | CategoryCreationRequest | Category | create new category |
| GET | `/category/{wid}` | - | Category | get category details |
| PUT | `/category/{wid}` | CategoryUpdateRequest | Category | update category details |
| DELETE | `/category/{wid}` | - | HTTP 200? | delete category |

### Transactions

| Method | Endpoint | Payload/Params | Result | Description |
| :--: | -- | -- | -- | -- |
| GET | `/transactions` | `from, to` | get transaction history |
| POST | `/transaction` | TransactionCreationRequest | create new transaction |
| GET | `/transaction/{tid}` | - | get transaction details |
| PUT | `/transaction/{tid}` | TransactionUpdateRequest | update transaction details |
| DELETE | `/transaction/{tid}` | - | delete transaction |

### Statistics
| Method | Endpoint | Payload/Params | Description |
| :--: | -- | -- | -- |
| GET | `/statistics` | `wallet_ids, from, to` | get general statistics for a list of wallets |
| GET | `/statistics/category` | `wallet_ids, from, to` | get category statistics for a list of wallets |
| GET | `/statistics/cashflow` | `wallet_ids, from, to` | get cashflow statistics for a list of wallets|
