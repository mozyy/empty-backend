// @generated automatically by Diesel CLI.

diesel::table! {
    client_scope (id) {
        id -> Int4,
        client_id -> Uuid,
        redirect_uri_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    clients (id) {
        id -> Uuid,
        redirect_uri_id -> Int4,
        name -> Text,
        desc -> Text,
        passphrase -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    redirect_uris (id) {
        id -> Int4,
        url -> Text,
        created_at -> Timestamp,
        client_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    scopes (id) {
        id -> Int4,
        name -> Text,
        desc -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(client_scope -> clients (client_id));
diesel::joinable!(client_scope -> redirect_uris (redirect_uri_id));

diesel::allow_tables_to_appear_in_same_query!(
    client_scope,
    clients,
    redirect_uris,
    scopes,
);
