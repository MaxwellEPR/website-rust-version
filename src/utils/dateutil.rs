use chrono::NaiveDateTime;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn getTimeStamp() -> u64 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("获取时间戳错误");
    duration.as_secs()
}

pub fn getNowAsFormat(form: &str) -> String {
    let timestamp = getTimeStamp() as i64;
    NaiveDateTime::from_timestamp_millis(timestamp)
        .unwrap()
        .to_string()
}
