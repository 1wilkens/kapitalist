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
        wallet_type     -> Text,
        initial_balance -> Int4,
        current_balance -> Int4,
        color           -> Text,
        created_at      -> Timestamp,
    }
}

table! {
    categories {
        id         -> Int4,
        parent_id  -> Nullable<Int4>,
        user_id    -> Nullable<Int4>,
        name       -> Text,
        color      -> Text,
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
// XXX: This seems to be invalid in diesel? Investigate..
// joinable!(categories -> categories (parent_id));
joinable!(categories -> users (user_id));
joinable!(transactions -> wallets (wallet_id));
joinable!(transactions -> categories (category_id));

allow_tables_to_appear_in_same_query!(users, wallets, categories, transactions);
