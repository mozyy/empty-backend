use diesel::{
    deserialize::{self, FromSql},
    pg::{Pg, PgValue},
    serialize::{self, IsNull, Output, ToSql},
    PgConnection, RunQueryDsl,
};
use empty_utils::errors::ServiceResult;

use crate::{pb, schema::lotterys};

impl ToSql<crate::schema::sql_types::Item, Pg> for pb::Item {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        todo!()
    }
}

impl FromSql<crate::schema::sql_types::Item, Pg> for pb::Item {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        todo!()
    }
}

impl ToSql<crate::schema::sql_types::Remark, Pg> for pb::Remark {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        todo!()
    }
}

impl FromSql<crate::schema::sql_types::Remark, Pg> for pb::Remark {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        todo!()
    }
}

impl FromIterator<Option<pb::Item>> for pb::Item {
    fn from_iter<T: IntoIterator<Item = Option<pb::Item>>>(iter: T) -> Self {
        todo!()
    }
}

// #[derive(
//     ::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable,
// )]
// pub struct Lottery {
//     pub id: i32,
//     pub title: ::prost::alloc::string::String,
//     #[diesel(column_name = "type_")]
//     pub r#type: i32,
//     // #[diesel(deserialize_as = Vec<Option<pb::Item>>)]
//     pub items: Vec<Option<pb::Item>>,
//     pub remark: bool,
//     pub remarks: Vec<Option<pb::Remark>>,
//     pub created_at: ::core::option::Option<::empty_utils::tonic::timestamp::Timestamp>,
//     pub updated_at: ::core::option::Option<::empty_utils::tonic::timestamp::Timestamp>,
// }

// pub async fn query_list(conn: &mut PgConnection) -> ServiceResult<Vec<Lottery>> {
//     let lotterys = lotterys::table.load::<Lottery>(conn)?;
//     Ok(lotterys)
// }
