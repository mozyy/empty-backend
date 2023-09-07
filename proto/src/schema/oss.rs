// @generated automatically by Diesel CLI.

diesel::table! {
    oss (id) {
        id -> Int4,
        bucket_name -> Int4,
        object_key -> Text,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
