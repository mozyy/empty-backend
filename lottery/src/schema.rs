// @generated automatically by Diesel CLI.

diesel::table! {
    items (id) {
        id -> Int4,
        name -> Text,
        value -> Int4,
        lottery_id -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    lotterys (id) {
        id -> Int4,
        title -> Text,
        #[sql_name = "type"]
        type_ -> Int4,
        remark -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    remarks (id) {
        id -> Int4,
        name -> Text,
        require -> Bool,
        lottery_id -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(items -> lotterys (lottery_id));
diesel::joinable!(remarks -> lotterys (lottery_id));

diesel::allow_tables_to_appear_in_same_query!(items, lotterys, remarks,);
