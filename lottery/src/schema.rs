// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "item"))]
    pub struct Item;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "oauth_pattern"))]
    pub struct OauthPattern;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "remark"))]
    pub struct Remark;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Item;
    use super::sql_types::Remark;

    lotterys (id) {
        id -> Int4,
        user_id -> Uuid,
        title -> Text,
        #[sql_name = "type"]
        type_ -> Int4,
        items -> Array<Item>,
        remark -> Bool,
        remarks -> Array<Remark>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    oauth_clients (id) {
        id -> Uuid,
        name -> Text,
        redirect_uri -> Text,
        default_scope -> Text,
        passdata -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::OauthPattern;

    oauth_configs (id) {
        id -> Int4,
        pattern -> Nullable<OauthPattern>,
        scope -> Nullable<Text>,
    }
}

diesel::table! {
    records (id) {
        id -> Int4,
        lottery_id -> Int4,
        user_id -> Uuid,
        value -> Text,
        remarks -> Array<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    wx_users (id) {
        id -> Uuid,
        user_id -> Uuid,
        openid -> Text,
        unionid -> Nullable<Text>,
        session_key -> Text,
        name -> Text,
        avatar -> Nullable<Text>,
        mobile -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(lotterys -> users (user_id));
diesel::joinable!(records -> lotterys (lottery_id));
diesel::joinable!(records -> users (user_id));
diesel::joinable!(wx_users -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    lotterys,
    oauth_clients,
    oauth_configs,
    records,
    users,
    wx_users,
);
