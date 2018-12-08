table! {
    users {
        id         -> Int4,
        email      -> Text,
        secret     -> Text,
        username   -> Text,
        created_at -> Timestamp,
    }
}

table! {
    wallets {
        id              -> Int4,
        user_id         -> Int4,
        name            -> Text,
        initial_balance -> Int4,
        current_balance -> Int4,
        color           -> Nullable<Text>,
        created_at      -> Timestamp,
    }
}

table! {
    categories {
        id         -> Int4,
        user_id    -> Int4,
        name       -> Text,
        color      -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

table! {
    transactions {
        id          -> Int4,
        wallet_id   -> Int4,
        category_id -> Int4,
        amount      -> Int4,
    }
}

// XXX: Maybe add more joinable macro calls
joinable!(wallets -> users (user_id));
joinable!(categories -> users (user_id));
joinable!(transactions -> wallets (wallet_id));
joinable!(transactions -> categories (category_id));

allow_tables_to_appear_in_same_query!(users, wallets, categories, transactions);
