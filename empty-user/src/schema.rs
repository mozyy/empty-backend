// @generated automatically by Diesel CLI.

diesel::table! {
    access_tokens (access_token) {
        access_token -> Text,
        info_id -> Uuid,
        scope -> Text,
        expires_in -> Int4,
        refresh_token -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

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
    refresh_tokens (refresh_token) {
        refresh_token -> Text,
        info_id -> Uuid,
        expires_in -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    scopes (id) {
        id -> Int4,
        scope -> Text,
        pattern -> Text,
        desc -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(access_tokens -> infos (info_id));
diesel::joinable!(access_tokens -> refresh_tokens (refresh_token));
diesel::joinable!(refresh_tokens -> infos (info_id));

diesel::allow_tables_to_appear_in_same_query!(access_tokens, infos, refresh_tokens, scopes,);
