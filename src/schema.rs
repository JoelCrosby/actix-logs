table! {
    users (id) {
        id -> Integer,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
    }
}

table! {
    log_entries (id) {
        id -> Integer,
        title -> Text,
        serialised -> Json,
        created_at -> Timestamp,
    }
}
