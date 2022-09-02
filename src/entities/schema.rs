table! {
    log_entries (id) {
        id -> Int4,
        title -> Varchar,
        serialised -> Text,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
        login_session -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(log_entries, users,);
