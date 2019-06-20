table! {
    users {
        id         -> Int8,
        email      -> Text,
        secret     -> Text,
        username   -> Text,
        created_at -> Timestamp,
    }
}

table! {
    wallets {
        id              -> Int8,
        user_id         -> Int8,
        name            -> Text,
        wallet_type     -> Text,
        initial_balance -> Int8,
        current_balance -> Int8,
        color           -> Text,
        created_at      -> Timestamp,
    }
}

table! {
    categories {
        id         -> Int8,
        parent_id  -> Nullable<Int8>,
        user_id    -> Nullable<Int8>,
        name       -> Text,
        color      -> Text,
        created_at -> Timestamp,
    }
}

table! {
    transactions (id) {
        name        -> Text,
        id          -> Int8,
        wallet_id   -> Int8,
        category_id -> Int8,
        amount      -> Int8,
        ts          -> Timestamp,
    }
}

// XXX: Maybe add more joinable macro calls
joinable!(wallets -> users (user_id));
// XXX: This seems to be invalid in diesel? Investigate..
// joinable!(categories -> categories (parent_id));
joinable!(categories -> users (user_id));
joinable!(transactions -> wallets (wallet_id));
joinable!(transactions -> categories (category_id));

allow_tables_to_appear_in_same_query!(users, wallets, categories, transactions);
