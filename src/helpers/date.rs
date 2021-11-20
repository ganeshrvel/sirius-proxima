use chrono::Utc;
use std::time::Instant;

pub fn _get_unix_timestamp_ms() -> i64 {
    let now = Utc::now();
    now.timestamp_millis()
}

pub fn _get_unix_timestamp_us() -> i64 {
    let now = Utc::now();
    now.timestamp_nanos()
}

pub fn _get_elapsed_time_ms(start: Instant) -> u128 {
    start.elapsed().as_millis()
}
