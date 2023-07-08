diesel::table! {
    todos (id) {
        id -> Int4,
        test -> Text,
        created_at -> Timestamp,
    }
}
