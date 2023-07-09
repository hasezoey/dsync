diesel::table! {
    todos (id) {
        id -> Int4,
        text -> Text,
    }
}

diesel::table! {
    view_todos (id) {
        id -> Int4,
        text -> Text,
    }
}
