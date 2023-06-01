// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "item"))]
    pub struct Item;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "remark"))]
    pub struct Remark;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Item;
    use super::sql_types::Remark;

    lotterys (id) {
        id -> Int4,
        title -> Text,
        #[sql_name = "type"]
        type_ -> Int4,
        items -> Array<Item>,
        remark -> Bool,
        remarks -> Array<Remark>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
