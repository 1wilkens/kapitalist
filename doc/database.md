# kapitalist - database

## System
- Paradigm
- Candidates

## Entities
- User
    - Id
    - EMail
    - Secret (salted & hashed?)
    - Name

- Wallet
    - Id
    - User_Id
    - Type*
    - InitialBalance
    - CurrentBalance?
    - Color*

- Transaction
    - Id
    - Timestamp
    - Wallet_Id
    - Name
    - Type
    - Amount
    - Category_Id

- Category
    - Id
    - Name
    - Color
    - Image (just unicode glyph for fontawesome?)

- SubCategory
    - Id
    - Name
    - Category_Id
    - Image (unicode..)
