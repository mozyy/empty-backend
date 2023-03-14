// @generated automatically by Diesel CLI.

diesel::table! {
    clients (id) {
        id -> Uuid,
        redirect_uri_id -> Int4,
        default_scope -> Array<Nullable<Text>>,
        client_type -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    registered_urls (id) {
        id -> Int4,
        client_id -> Nullable<Uuid>,
        url -> Text,
        #[sql_name = "type"]
        type_ -> Int2,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    clients,
    registered_urls,
);
