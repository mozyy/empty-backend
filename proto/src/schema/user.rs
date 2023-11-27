// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "pattern"))]
    pub struct Pattern;
}

diesel::table! {
    authorization_codes (id) {
        id -> Int4,
        code -> Text,
        user_id -> Uuid,
        client_id -> Uuid,
        scope -> Text,
        redirect_uri -> Text,
        until -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    clients (id) {
        id -> Uuid,
        name -> Text,
        redirect_uri -> Text,
        default_scope -> Text,
        default_expires_in -> Int4,
        passdata -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Pattern;

    configs (id) {
        id -> Int4,
        pattern -> Nullable<Pattern>,
        scope -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    infos (id) {
        id -> Int4,
        name -> Text,
        avatar -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    mobiles (id) {
        id -> Int4,
        mobile -> Text,
        password -> Text,
        salt -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    refresh_resources (id) {
        id -> Int4,
        user_id -> Uuid,
        client_id -> Uuid,
        refresh_token -> Text,
        scope -> Text,
        token_type -> Text,
        until -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        info_id -> Int4,
        mobile_id -> Nullable<Int4>,
        weixin_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    weixins (id) {
        id -> Int4,
        openid -> Text,
        unionid -> Nullable<Text>,
        session_key -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(authorization_codes -> clients (client_id));
diesel::joinable!(authorization_codes -> users (user_id));
diesel::joinable!(refresh_resources -> clients (client_id));
diesel::joinable!(refresh_resources -> users (user_id));
diesel::joinable!(users -> infos (info_id));
diesel::joinable!(users -> mobiles (mobile_id));
diesel::joinable!(users -> weixins (weixin_id));

diesel::allow_tables_to_appear_in_same_query!(
    authorization_codes,
    clients,
    configs,
    infos,
    mobiles,
    refresh_resources,
    users,
    weixins,
);
