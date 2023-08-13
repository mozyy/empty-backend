// @generated automatically by Diesel CLI.

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

diesel::joinable!(items -> lotterys (lottery_id));
diesel::joinable!(record_remarks -> records (record_id));
diesel::joinable!(record_remarks -> remarks (remark_id));
diesel::joinable!(records -> items (item_id));
diesel::joinable!(records -> lotterys (lottery_id));
diesel::joinable!(remarks -> lotterys (lottery_id));

diesel::allow_tables_to_appear_in_same_query!(items, lotterys, record_remarks, records, remarks,);
