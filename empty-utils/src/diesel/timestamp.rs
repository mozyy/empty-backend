use chrono::{NaiveDateTime, Timelike};
use diesel::{
    deserialize::{self, FromSql},
    pg::{Pg, PgValue},
    serialize::{self, Output, ToSql},
    sql_types, AsExpression, FromSqlRow,
};
use serde::{Deserialize, Deserializer, Serializer};
// The signature of a serialize_with function must follow the pattern:
//
//    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
//    where
//        S: Serializer
//
// although it may also be generic over the input types T.
pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i64(date.timestamp_millis())
}

// The signature of a deserialize_with function must follow the pattern:
//
//    fn deserialize<'de, D>(D) -> Result<T, D::Error>
//    where
//        D: Deserializer<'de>
//
// although it may also be generic over the output types T.
pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let value = i64::deserialize(deserializer)?;
    NaiveDateTime::from_timestamp_opt(value / 1000, ((value % 1000) * 1_000_000) as u32)
        .ok_or_else(|| serde::de::Error::custom(format!("deserial timestamp error: {}", value)))
}

// time
#[derive(Clone, Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type = sql_types::Timestamp)]
pub struct Timestamp(prost_types::Timestamp);

impl From<NaiveDateTime> for Timestamp {
    fn from(value: NaiveDateTime) -> Self {
        Self(prost_types::Timestamp {
            seconds: value.timestamp(),
            nanos: value.nanosecond() as i32,
        })
    }
}
impl From<Timestamp> for NaiveDateTime {
    fn from(value: Timestamp) -> Self {
        Self::from_timestamp_opt(value.0.seconds, value.0.nanos as u32).unwrap()
    }
}

impl From<Timestamp> for Option<prost_types::Timestamp> {
    fn from(value: Timestamp) -> Self {
        Some(value.0)
    }
}

impl FromSql<sql_types::Timestamp, Pg> for Timestamp {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let time: deserialize::Result<NaiveDateTime> =
            FromSql::<sql_types::Timestamp, Pg>::from_sql(bytes);
        time.map(|t| t.into())
    }
}

impl ToSql<sql_types::Timestamp, Pg> for Timestamp {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        ToSql::<sql_types::Timestamp, Pg>::to_sql(
            &NaiveDateTime::from(self.to_owned()),
            &mut out.reborrow(),
        )
    }
}
