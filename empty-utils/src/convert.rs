use chrono::{NaiveDateTime, Timelike};
use prost_types::Timestamp;

pub fn naiveDateTimeToTimestamp(time: NaiveDateTime) -> Timestamp {
    Timestamp {
        seconds: time.second() as i64,
        nanos: time.nanosecond() as i32,
    }
}
