// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
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
