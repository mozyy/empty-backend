// @generated automatically by Diesel CLI.

diesel::table! {
    blogs (id) {
        id -> Int4,
        title -> Text,
        image -> Text,
        summary -> Text,
        markdown -> Text,
        author -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
