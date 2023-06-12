use diesel::{
    deserialize::{self, FromSql},
    pg::{Pg, PgValue},
    serialize::{self, Output, ToSql, WriteTuple},
    sql_types::{Bool, Integer, Record, Text},
};

use crate::pb;

impl ToSql<crate::schema::sql_types::Item, Pg> for pb::Item {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        WriteTuple::<(Text, Integer)>::write_tuple(&(self.name.to_owned(), self.value), out)
    }
}

impl FromSql<crate::schema::sql_types::Item, Pg> for pb::Item {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let (name, value) = FromSql::<Record<(Text, Integer)>, Pg>::from_sql(bytes)?;

        Ok(Self { name, value })
    }
}

impl ToSql<crate::schema::sql_types::Remark, Pg> for pb::Remark {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        WriteTuple::<(Text, Bool)>::write_tuple(&(self.name.to_owned(), self.require), out)
    }
}

impl FromSql<crate::schema::sql_types::Remark, Pg> for pb::Remark {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let (name, require) = FromSql::<Record<(Text, Bool)>, Pg>::from_sql(bytes)?;

        Ok(Self { name, require })
    }
}

impl FromIterator<Option<pb::Item>> for pb::Item {
    fn from_iter<T: IntoIterator<Item = Option<pb::Item>>>(_iter: T) -> Self {
        todo!()
    }
}
