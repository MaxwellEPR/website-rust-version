// use actix_easy_multipart::{MultipartForm, tempfile::Tempfile};
// use actix_multipart::MultipartError;
// use actix_multipart::form::MultipartForm;
// use actix_multipart::form::{tempfile::TempFile, MultipartCollect,State};
use chrono::NaiveDateTime;
use r2d2_redis::redis::{from_redis_value, FromRedisValue, RedisResult, ToRedisArgs, Value};
use rand::random;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display, time::SystemTime, time::UNIX_EPOCH, fs::File};
use validator::{Validate, ValidationError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Submit,
    Processing,
    Complete,
    Fail,
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        match self {
            Self::Submit => s.push_str("Submit"),
            Self::Processing => s.push_str("Processing"),
            Self::Complete => s.push_str("Complete"),
            Self::Fail => s.push_str("Fail"),
        }
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TaskBody {
    pub task_id: String,
    pub status: TaskStatus,
    #[validate(email)]
    pub email: String,
    pub model_name: String,
    #[serde(skip_serializing)]
    pub uuid: String,
    #[serde(skip_serializing)]
    pub captcha: u8,
    pub gen_pair: usize,
    submit_time: String,
    complete_time: String,
    #[validate(custom = "validate_data")]
    pub data: Vec<String>,
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
            status: TaskStatus::Submit,
            email: String::from(""),
            model_name: String::from(""),
            uuid: String::from(""),
            captcha: 0,
            gen_pair: 0,
            submit_time: format_time,
            complete_time: String::from(""),
            data: Vec::new(),
        }
    }
}

impl Display for TaskBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "task_id:{},status:{},email:{},model_name:{},uuid:{},captcha:{},gen_pair:{},submit_time:{},complete_time:{},data:{}",
            self.task_id,
            self.status,
            self.email,
            self.model_name,
            self.uuid,
            self.captcha,
            self.gen_pair,
            self.submit_time,
            self.complete_time,
            self.data.iter().map(|s|format!("{}",s)).collect::<String>()
        )
    }
}

impl FromRedisValue for TaskBody {
    fn from_redis_value(v: &r2d2_redis::redis::Value) -> RedisResult<Self> {
        let v: String = from_redis_value(v)?;
        RedisResult::Ok(serde_json::from_str::<TaskBody>(&v).expect("get task_body fail"))
    }

    fn from_redis_values(items: &[Value]) -> RedisResult<Vec<Self>> {
        let v = items
            .iter()
            .map(|val| from_redis_value(val).unwrap())
            .collect::<Vec<String>>();
        let res = v
            .iter()
            .map(|val| serde_json::from_str::<TaskBody>(&val).expect("get task body fail"))
            .collect::<Vec<TaskBody>>();
        RedisResult::Ok(res)
    }
}

impl ToRedisArgs for TaskBody {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + r2d2_redis::redis::RedisWrite,
    {
        out.write_arg_fmt(self);
    }
}


fn validate_data(data: &Vec<String>) -> Result<(), ValidationError> {
    if data.len().eq(&0) {
        return Ok(());
    }
    let ppi = Regex::new(r"^>\w{1,20}\n[aAtTcCgGnN]+$").unwrap();
    if data.iter().all(|s| ppi.is_match(s)) {
        return Ok(());
    };
    Err(ValidationError::new("ppi格式错误"))
}

#[derive(Debug, Serialize)]
pub struct TaskResponse {
    pub task_body: TaskBody,
    pub csv_content: Option<HashMap<String, HashMap<String, Vec<String>>>>,
    pub heat_map: Option<HashMap<String, Vec<Vec<f32>>>>,
}

impl TaskResponse {
    pub fn new(
        task_body: TaskBody,
        csv_content: Option<HashMap<String, HashMap<String, Vec<String>>>>,
        heat_map: Option<HashMap<String, Vec<Vec<f32>>>>,
    ) -> TaskResponse {
        TaskResponse {
            task_body,
            csv_content,
            heat_map,
        }
    }
}

// #[derive(Debug)]
// pub struct TaskFile {
//     task_body: TaskBody,
//     file: Tempfile,
// }

// impl TaskFile {
//     pub fn new(task_body: TaskBody, file: Tempfile) -> TaskFile {
//         TaskFile { task_body, file }
//     }
// }

#[cfg(test)]
mod test {
    use validator::ValidationError;

    use super::validate_data;

    #[test]
    pub fn test_new() {
        use crate::entity::task_body::TaskBody;
        let task_body = TaskBody::new();
        println!("{}", task_body)
    }

    #[test]
    pub fn test_custom_validation() {
        let data = vec![">zmoajfeoaj\nAAACCCCCaaaCCCcccttTTTgggGGGNNnnn".to_string()];
        assert_eq!(validate_data(&data), Ok(()));
        let wrong = vec![">faod\noawefujoaj".to_string()];
        assert_eq!(
            validate_data(&wrong),
            Err(ValidationError::new("ppi格式错误"))
        );
    }
}
