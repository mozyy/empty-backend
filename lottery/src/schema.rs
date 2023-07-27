// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "oauth_pattern"))]
    pub struct OauthPattern;
}

diesel::table! {
    items (id) {
        id -> Int4,
        lottery_id -> Int4,
        name -> Text,
        value -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    lotterys (id) {
        id -> Int4,
        user_id -> Uuid,
        title -> Text,
        #[sql_name = "type"]
        type_ -> Int4,
        remark -> Bool,
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
    record_remarks (id) {
        id -> Int4,
        record_id -> Int4,
        remark_id -> Int4,
        value -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    records (id) {
        id -> Int4,
        lottery_id -> Int4,
        user_id -> Uuid,
        item_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    remarks (id) {
        id -> Int4,
        lottery_id -> Int4,
        name -> Text,
        require -> Bool,
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

diesel::joinable!(items -> lotterys (lottery_id));
diesel::joinable!(lotterys -> users (user_id));
diesel::joinable!(record_remarks -> records (record_id));
diesel::joinable!(record_remarks -> remarks (remark_id));
diesel::joinable!(records -> items (item_id));
diesel::joinable!(records -> lotterys (lottery_id));
diesel::joinable!(records -> users (user_id));
diesel::joinable!(remarks -> lotterys (lottery_id));
diesel::joinable!(wx_users -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    items,
    lotterys,
    oauth_clients,
    oauth_configs,
    record_remarks,
    records,
    remarks,
    users,
    wx_users,
);
