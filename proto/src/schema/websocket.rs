// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        addr -> Text,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
