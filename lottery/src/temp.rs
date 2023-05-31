use diesel::{PgConnection, prelude::*};
use empty_utils::errors::ServiceResult;

use crate::schema::lotterys;

#[derive(
    ::diesel::prelude::Queryable,
    ::diesel::prelude::Associations,
    // ::diesel::prelude::Identifiable,
    ::diesel::prelude::Selectable
)]
#[diesel(table_name = crate::schema::items)]
#[diesel(belongs_to(LotteryInfo, foreign_key = lottery_id))]
pub struct Item {
    /// id
    pub id: i32,
    /// lottery title
    pub name: ::prost::alloc::string::String,
    /// lottery summary
    pub value: i32,
    /// id
    pub lottery_id: i32,
    pub created_at: ::core::option::Option<::empty_utils::tonic::timestamp::Timestamp>,
    pub updated_at: ::core::option::Option<::empty_utils::tonic::timestamp::Timestamp>,
}

#[derive(
    ::diesel::prelude::Queryable,
    ::diesel::prelude::Identifiable,
    ::diesel::prelude::Selectable
)]
#[diesel(table_name = crate::schema::lotterys)]
pub struct LotteryInfo {
    /// id
    pub id: i32,
    /// lottery title
    pub title: ::prost::alloc::string::String,
    /// lottery image
    #[diesel(column_name = "type_")]
    pub r#type: i32,
    /// lottery summary
    pub remark: bool,
    pub created_at: ::core::option::Option<::empty_utils::tonic::timestamp::Timestamp>,
    pub updated_at: ::core::option::Option<::empty_utils::tonic::timestamp::Timestamp>,
}

pub async fn query_list(conn: &mut PgConnection) -> ServiceResult<Vec<(LotteryInfo, Item)>> {
    let lotterys = lotterys::table.load::<LotteryInfo>(conn)?;

    let items: Vec<Vec<Item>> = Item::belonging_to(&lotterys)
        .load::<Item>(conn)?
        .grouped_by(&lotterys);

    let books_per_author: Vec<Vec<Item>> = items.grouped_by(&lotterys);

    todo!();
}