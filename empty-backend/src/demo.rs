mod demo {
    diesel::table! {
        answers (id) {
            id -> Int4,
            content -> Text,
            correct -> Bool,
            created_at -> Timestamp,
            updated_at -> Timestamp,
        }
    }
}
