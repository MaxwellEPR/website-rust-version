use chrono::NaiveDateTime;
use rand::random;
use serde::Serialize;
use std::{collections::HashMap, fmt::Display, time::SystemTime, time::UNIX_EPOCH};

#[derive(Debug, Clone, Serialize)]
pub struct TaskBody {
    task_id: String,
    pub status: u8,
    pub email: String,
    pub model_name: String,
    #[serde(skip_serializing)]
    pub uuid: String,
    #[serde(skip_serializing)]
    pub captcha: u8,
    submit_time: String,
    complete_time: String,
    pub data: String,
}

impl TaskBody {
    pub fn new() -> TaskBody {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("时间解析出错");
        let timestamp = duration.as_secs();
        let task_id = format!("{}_{}", random::<u16>() % 1000 + 1, timestamp);
        let format_time = NaiveDateTime::from_timestamp_millis(timestamp as i64)
            .unwrap()
            .to_string();

        TaskBody {
            task_id,
            status: 0,
            email: String::from(""),
            model_name: String::from(""),
            uuid: String::from(""),
            captcha: 0,
            submit_time: format_time,
            complete_time: String::from(""),
            data: String::from(""),
        }
    }
}

impl Display for TaskBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{},{},{},{},{},{}",
            self.task_id,
            self.status,
            self.email,
            self.model_name,
            self.uuid,
            self.captcha,
            self.submit_time,
            self.complete_time,
            self.data
        )
    }
}

pub struct TaskResponse {
    task_body: TaskBody,
    result: HashMap<String, HashMap<String, Vec<String>>>,
}

impl TaskResponse {
    pub fn new(
        task_body: TaskBody,
        result: HashMap<String, HashMap<String, Vec<String>>>,
    ) -> TaskResponse {
        TaskResponse { task_body, result }
    }
}

#[cfg(test)]
mod test {

    #[test]
    pub fn test_new() {
        use crate::entity::TaskBody;
        let taskBody = TaskBody::new();
        println!("{}", taskBody)
    }
}
