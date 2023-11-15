use chrono::NaiveDateTime;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_timestamp() -> u64 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("获取时间戳错误");
    duration.as_secs()
}

pub fn now_as_format(form: &str) -> String {
    let timestamp = get_timestamp() as i64;
    NaiveDateTime::from_timestamp_millis(timestamp)
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod test{
    use crate::utils::dateutil::{get_timestamp, now_as_format};

    #[test]
    pub fn test_get_timestamp(){
        println!("{}",get_timestamp());
    }

    #[test]
    pub fn test_now_as_format(){
        let form = "yyyy-MM-dd";
        println!("{}",now_as_format(form));
    }
}