// @generated automatically by Diesel CLI.

diesel::table! {
    infos (id) {
        id -> Uuid,
        mobile -> Text,
        password -> Text,
        username -> Nullable<Text>,
        avatar -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    tokens (access_token) {
        access_token -> Text,
        info_id -> Uuid,
        expires_in -> Int4,
        refresh_token -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(tokens -> infos (info_id));

diesel::allow_tables_to_appear_in_same_query!(
    infos,
    tokens,
);
