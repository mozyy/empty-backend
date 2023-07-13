use diesel::{
    deserialize::{self, FromSql},
    pg::{Pg, PgValue},
    serialize::{self, Output, ToSql, WriteTuple},
    sql_types::{Integer, Record, Text},
};

use crate::pb::oauth as pb;

impl ToSql<crate::schema::sql_types::OauthPattern, Pg> for pb::Pattern {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let tuple = match self.to_owned().pattern {
            Some(pattern) => match pattern {
                pb::pattern::Pattern::Equal(value) => (1, value),
                pb::pattern::Pattern::Prefix(value) => (2, value),
                pb::pattern::Pattern::Regex(value) => (3, value),
            },
            None => return Err("no pattern".into()),
        };
        WriteTuple::<(Integer, Text)>::write_tuple(&tuple, out)
    }
}

impl FromSql<crate::schema::sql_types::OauthPattern, Pg> for pb::Pattern {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let (type_, value) = FromSql::<Record<(Integer, Text)>, Pg>::from_sql(bytes)?;
        let pattern = match type_ {
            1 => Some(pb::pattern::Pattern::Equal(value)),
            2 => Some(pb::pattern::Pattern::Prefix(value)),
            3 => Some(pb::pattern::Pattern::Regex(value)),
            _ => {
                log::warn!("database no pattern");
                None
            }
        };

        Ok(Self { pattern })
    }
}
