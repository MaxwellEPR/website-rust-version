use chrono::{DateTime, NaiveDate, TimeZone, Date, Utc};

pub struct ModelEntity {
    id: usize,
    model_name: String,
    pycode: String,
    vir_env_name: String,
    create_time: NaiveDate,
    last_modified_time: NaiveDate,
}

impl ModelEntity {
    pub fn new() -> ModelEntity {
        ModelEntity {
            id: 0,
            model_name: String::from(""),
            pycode: String::from(""),
            vir_env_name: String::from(""),
            create_time: Utc::now().date_naive(),
            last_modified_time: Utc::now().date_naive(),
        }
    }
}


