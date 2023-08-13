// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "oauth_pattern"))]
    pub struct OauthPattern;
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
    users (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(oauth_clients, oauth_configs, users,);
