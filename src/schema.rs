table! {
    users (id) {
        id -> Int4,
        email -> Text,
        secret_hash -> Text,
        username -> Text,
        created_at -> Timestamp,
    }
}

table! {
    wallets (id) {
        id -> Int4,
        name -> Text,
        initial_balance -> Int4,
        current_balance -> Int4,
        color -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

table! {
    categories (id) {
        id -> Int4,
        name -> Text,
        color -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    users,
    wallets,
    categories,
);