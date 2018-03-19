table! {
    users (id) {
        id -> Int4,
        email -> Text,
        secret_hash -> Text,
        username -> Text,
        creation_date -> Timestamp,
    }
}