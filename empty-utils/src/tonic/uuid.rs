use diesel::{
    deserialize::{self, FromSql},
    pg::{Pg, PgValue},
    serialize::{self, Output, ToSql},
    sql_types, AsExpression, FromSqlRow,
};

#[derive(Clone, Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type = sql_types::Uuid)]
pub struct Uuid(uuid::Uuid);

impl From<String> for Uuid {
    fn from(value: String) -> Self {
        Self(uuid::Uuid::parse_str(value.as_str()).unwrap())
    }
}
impl From<Uuid> for String {
    fn from(value: Uuid) -> Self {
        value.0.to_string()
    }
}

impl FromSql<sql_types::Uuid, Pg> for Uuid {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let value: deserialize::Result<uuid::Uuid> =
            FromSql::<sql_types::Uuid, Pg>::from_sql(bytes);
        value.map(Self)
    }
}

impl ToSql<sql_types::Uuid, Pg> for Uuid {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        ToSql::<sql_types::Uuid, Pg>::to_sql(&self.0, out)
    }
}
