use diesel::{
    deserialize::{self, FromSql},
    pg::{Pg, PgValue},
    serialize::{self, Output, ToSql, WriteTuple},
    sql_types::{Integer, Record, Text},
};

use crate::{pb, schema};

impl ToSql<schema::user::sql_types::Pattern, Pg> for pb::user::auth::Pattern {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let tuple = match self.to_owned().pattern {
            Some(pattern) => match pattern {
                pb::user::auth::pattern::Pattern::Equal(value) => (1, value),
                pb::user::auth::pattern::Pattern::Prefix(value) => (2, value),
                pb::user::auth::pattern::Pattern::Regex(value) => (3, value),
            },
            None => return Err("no pattern".into()),
        };
        WriteTuple::<(Integer, Text)>::write_tuple(&tuple, out)
    }
}

impl FromSql<schema::user::sql_types::Pattern, Pg> for pb::user::auth::Pattern {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let (type_, value) = FromSql::<Record<(Integer, Text)>, Pg>::from_sql(bytes)?;
        let pattern = match type_ {
            1 => Some(pb::user::auth::pattern::Pattern::Equal(value)),
            2 => Some(pb::user::auth::pattern::Pattern::Prefix(value)),
            3 => Some(pb::user::auth::pattern::Pattern::Regex(value)),
            _ => {
                log::warn!("database no pattern");
                None
            }
        };

        Ok(Self { pattern })
    }
}
