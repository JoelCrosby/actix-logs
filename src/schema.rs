table! {
    logs (id) {
        id -> Int4,
        title -> Varchar,
        serialised -> Nullable<Json>,
        created_at -> Timestamp,
    }
}
