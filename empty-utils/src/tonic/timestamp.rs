use chrono::{NaiveDate, NaiveDateTime, Timelike};
use diesel::{
    data_types::PgTimestamp,
    deserialize::{self, FromSql},
    pg::{Pg, PgValue},
    serialize::{self, Output, ToSql},
    sql_types, AsExpression, FromSqlRow,
};

// ----------------------- https://github.com/tokio-rs/prost/blob/master/prost-types/src/protobuf.rs#L2296

// #[derive(AsExpression, FromSqlRow, Debug, Clone, PartialEq)]
// #[diesel(sql_type = sql_types::Timestamp)]
// #[diesel(sql_type = sql_types::Timestamptz)]
// pub struct Timestampp(chrono::NaiveDateTime);

/// A Timestamp represents a point in time independent of any time zone or local
/// calendar, encoded as a count of seconds and fractions of seconds at
/// nanosecond resolution. The count is relative to an epoch at UTC midnight on
/// January 1, 1970, in the proleptic Gregorian calendar which extends the
/// Gregorian calendar backwards to year one.
///
/// All minutes are 60 seconds long. Leap seconds are "smeared" so that no leap
/// second table is needed for interpretation, using a [24-hour linear
/// smear](<https://developers.google.com/time/smear>).
///
/// The range is from 0001-01-01T00:00:00Z to 9999-12-31T23:59:59.999999999Z. By
/// restricting to that range, we ensure that we can convert to and from [RFC
/// 3339](<https://www.ietf.org/rfc/rfc3339.txt>) date strings.
///
/// # Examples
///
/// Example 1: Compute Timestamp from POSIX `time()`.
///
/// ```text
/// Timestamp timestamp;
/// timestamp.set_seconds(time(NULL));
/// timestamp.set_nanos(0);
/// ```
///
/// Example 2: Compute Timestamp from POSIX `gettimeofday()`.
///
/// ```text
/// struct timeval tv;
/// gettimeofday(&tv, NULL);
///
/// Timestamp timestamp;
/// timestamp.set_seconds(tv.tv_sec);
/// timestamp.set_nanos(tv.tv_usec * 1000);
/// ```
///
/// Example 3: Compute Timestamp from Win32 `GetSystemTimeAsFileTime()`.
///
/// ```text
/// FILETIME ft;
/// GetSystemTimeAsFileTime(&ft);
/// UINT64 ticks = (((UINT64)ft.dwHighDateTime) << 32) | ft.dwLowDateTime;
///
/// // A Windows tick is 100 nanoseconds. Windows epoch 1601-01-01T00:00:00Z
/// // is 11644473600 seconds before Unix epoch 1970-01-01T00:00:00Z.
/// Timestamp timestamp;
/// timestamp.set_seconds((INT64) ((ticks / 10000000) - 11644473600LL));
/// timestamp.set_nanos((INT32) ((ticks % 10000000) * 100));
/// ```
///
/// Example 4: Compute Timestamp from Java `System.currentTimeMillis()`.
///
/// ```text
/// long millis = System.currentTimeMillis();
///
/// Timestamp timestamp = Timestamp.newBuilder().setSeconds(millis / 1000)
///      .setNanos((int) ((millis % 1000) * 1000000)).build();
/// ```
///
/// Example 5: Compute Timestamp from Java `Instant.now()`.
///
/// ```text
/// Instant now = Instant.now();
///
/// Timestamp timestamp =
///      Timestamp.newBuilder().setSeconds(now.getEpochSecond())
///          .setNanos(now.getNano()).build();
/// ```
///
/// Example 6: Compute Timestamp from current time in Python.
///
/// ```text
/// timestamp = Timestamp()
/// timestamp.GetCurrentTime()
/// ```
///
/// # JSON Mapping
///
/// In JSON format, the Timestamp type is encoded as a string in the
/// [RFC 3339](<https://www.ietf.org/rfc/rfc3339.txt>) format. That is, the
/// format is "{year}-{month}-{day}T{hour}:{min}:{sec}\\[.{frac_sec}\\]Z"
/// where {year} is always expressed using four digits while {month}, {day},
/// {hour}, {min}, and {sec} are zero-padded to two digits each. The fractional
/// seconds, which can go up to 9 digits (i.e. up to 1 nanosecond resolution),
/// are optional. The "Z" suffix indicates the timezone ("UTC"); the timezone
/// is required. A proto3 JSON serializer should always use UTC (as indicated by
/// "Z") when printing the Timestamp type and a proto3 JSON parser should be
/// able to accept both UTC and other timezones (as indicated by an offset).
///
/// For example, "2017-01-15T01:30:15.01Z" encodes 15.01 seconds past
/// 01:30 UTC on January 15, 2017.
///
/// In JavaScript, one can convert a Date object to this format using the
/// standard
/// \[toISOString()\](<https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toISOString>)
/// method. In Python, a standard `datetime.datetime` object can be converted
/// to this format using
/// \[`strftime`\](<https://docs.python.org/2/library/time.html#time.strftime>) with
/// the time format spec '%Y-%m-%dT%H:%M:%S.%fZ'. Likewise, in Java, one can use
/// the Joda Time's \[`ISODateTimeFormat.dateTime()`\](<http://www.joda.org/joda-time/apidocs/org/joda/time/format/ISODateTimeFormat.html#dateTime%2D%2D>) to obtain a formatter capable of generating timestamps in this format.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, AsExpression, FromSqlRow)]
#[diesel(sql_type = sql_types::Timestamp)]
#[diesel(sql_type = sql_types::Timestamptz)]
pub struct Timestamp {
    /// Represents seconds of UTC time since Unix epoch
    /// 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to
    /// 9999-12-31T23:59:59Z inclusive.
    #[prost(int64, tag = "1")]
    pub seconds: i64,
    /// Non-negative fractions of a second at nanosecond resolution. Negative
    /// second values with fractions must still have non-negative nanos values
    /// that count forward in time. Must be from 0 to 999,999,999
    /// inclusive.
    #[prost(int32, tag = "2")]
    pub nanos: i32,
}

impl From<Timestamp> for chrono::NaiveDateTime {
    fn from(value: Timestamp) -> Self {
        Self::from_timestamp_opt(value.seconds, value.nanos as u32).unwrap()
    }
}
impl From<chrono::NaiveDateTime> for Timestamp {
    fn from(time: chrono::NaiveDateTime) -> Self {
        Self {
            seconds: time.timestamp(),
            nanos: time.nanosecond() as i32,
        }
    }
}

// ------------------------ https://github.com/diesel-rs/diesel/blob/master/diesel/src/pg/types/date_and_time/chrono.rs#L14
// Postgres timestamps start from January 1st 2000.
fn pg_epoch() -> NaiveDateTime {
    NaiveDate::from_ymd_opt(2000, 1, 1)
        .expect("This is in supported range of chrono dates")
        .and_hms_opt(0, 0, 0)
        .expect("This is a valid input")
}

impl FromSql<sql_types::Timestamp, Pg> for Timestamp {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let time: deserialize::Result<NaiveDateTime> =
            FromSql::<sql_types::Timestamp, Pg>::from_sql(bytes);
        time.map(|t| t.into())
        // let PgTimestamp(offset) = FromSql::<sql_types::Timestamp, Pg>::from_sql(bytes)?;
        // match pg_epoch().checked_add_signed(Duration::microseconds(offset)) {
        //     Some(v) => Ok(v.into()),
        //     None => {
        //         let message = "Tried to deserialize a timestamp that is too large for Chrono";
        //         Err(message.into())
        //     }
        // }
    }
}

impl ToSql<sql_types::Timestamp, Pg> for Timestamp {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let time = match (NaiveDateTime::from(self.to_owned()).signed_duration_since(pg_epoch()))
            .num_microseconds()
        {
            Some(time) => time,
            None => {
                let error_message =
                    format!("{self:?} as microseconds is too large to fit in an i64");
                return Err(error_message.into());
            }
        };
        ToSql::<sql_types::Timestamp, Pg>::to_sql(&PgTimestamp(time), &mut out.reborrow())
    }
}

impl FromSql<sql_types::Timestamptz, Pg> for Timestamp {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        FromSql::<sql_types::Timestamp, Pg>::from_sql(bytes)
    }
}

impl ToSql<sql_types::Timestamptz, Pg> for Timestamp {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        ToSql::<sql_types::Timestamp, Pg>::to_sql(self, out)
    }
}
