use chrono::{NaiveDateTime, Timelike};
use prost_types::Timestamp;

pub fn naive_date_time_to_timestamp(time: NaiveDateTime) -> Timestamp {
    Timestamp {
        seconds: time.timestamp() as i64,
        nanos: time.nanosecond() as i32,
    }
}
