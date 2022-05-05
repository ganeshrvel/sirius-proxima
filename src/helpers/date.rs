use crate::DefaultValues;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use std::fmt::Debug;
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

pub fn utc_to_default_tz(date: DateTime<Utc>) -> DateTime<Tz> {
    let tz: Tz = DefaultValues::DEFAULT_TIMEZONE;

    date.with_timezone(&tz)
}

pub fn get_time_now_for_default_tz() -> DateTime<Tz> {
    let time_now = chrono::Utc::now();

    utc_to_default_tz(time_now)
}

pub fn get_formatted_notification_date_time(time: &DateTime<Tz>) -> String {
    time.format("%I:%M:%S %P, %v").to_string()
}

pub fn get_formatted_date_time(time: &DateTime<Tz>) -> String {
    time.format("%v %I:%M:%S %P").to_string()
}

pub fn get_formatted_date_time_now() -> String {
    let time_now = get_time_now_for_default_tz();
    get_formatted_date_time(&time_now)
}
