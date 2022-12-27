// @generated automatically by Diesel CLI.

diesel::table! {
    answers (id) {
        id -> Int4,
        question_id -> Int4,
        content -> Text,
        correct -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

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
    questions (id) {
        id -> Int4,
        content -> Text,
        desc -> Nullable<Text>,
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
    resources (id) {
        id -> Int4,
        resource_id -> Int4,
        key -> Varchar,
        rtype -> Int4,
        name -> Text,
        desc -> Nullable<Text>,
        sort -> Int4,
        path -> Text,
        index -> Nullable<Bool>,
        menu -> Nullable<Bool>,
        icon -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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

diesel::joinable!(answers -> questions (question_id));
diesel::joinable!(client_scope -> clients (client_id));
diesel::joinable!(client_scope -> redirect_uris (redirect_uri_id));

diesel::allow_tables_to_appear_in_same_query!(
    answers,
    client_scope,
    clients,
    questions,
    redirect_uris,
    resources,
    scopes,
);
