use sea_orm::{Database, DatabaseConnection, DbErr};

// static S_CONN: Result<DatabaseConnection, ()> = Err(());

pub struct DataSource {
    user: &'static str,
    password: &'static str,
    db_name: &'static str,
    ip: &'static str,
    port: u16,
    database: &'static str,
}

impl DataSource {
    pub fn new(
        user: &'static str,
        password: &'static str,
        db_name: &'static str,
        ip: &'static str,
        port: u16,
        database: &'static str,
    ) -> DataSource {
        DataSource {
            user,
            password,
            db_name,
            ip,
            port,
            database,
        }
    }

    pub fn get_url(&self) -> String {
        format!(
            "{}://{}:{}@{}:{}/{}",
            self.db_name, self.user, self.password, self.ip, self.port, self.database
        )
    }
}

pub async fn get_connection() -> Result<DatabaseConnection, DbErr> {
    let data_source = DataSource::new("root", "123456", "mysql", "localhost", 3306, "web");
    let db = Database::connect(data_source.get_url()).await?;
    return Ok(db);
}

#[cfg(test)]
mod test {
    use super::get_connection;

    #[test]
    pub fn test_connection() {
        // get_connection();
    }
}
