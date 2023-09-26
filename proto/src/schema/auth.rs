// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "pattern"))]
    pub struct Pattern;
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
    resources (id) {
        id -> Int4,
        user_id -> Uuid,
        client_id -> Uuid,
        access_token -> Text,
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
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(resources -> clients (client_id));
diesel::joinable!(resources -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(clients, configs, resources, users,);
