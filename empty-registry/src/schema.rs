// @generated automatically by Diesel CLI.

diesel::table! {
    micro_services (id) {
        id -> Uuid,
        name -> Text,
        endpoint -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
