table! {
    users (id) {
        id                   -> Int4,
        email                -> Text,
        password_fingerprint -> Text,
        username             -> Text,
        created_at           -> Timestamp,
    }
}

table! {
    wallets (id) {
        id              -> Int4,
        name            -> Text,
        initial_balance -> Int4,
        current_balance -> Int4,
        color           -> Nullable<Text>,
        created_at      -> Timestamp,
    }
}

table! {
    categories (id) {
        id         -> Int4,
        name       -> Text,
        color      -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

// XXX: Maybe add useful joinable macro calls

allow_tables_to_appear_in_same_query!(users, wallets, categories,);
