use mysql::{Error, Pool};

pub fn get_mysql_pool(user: &str, password: &str, db_name: &str) -> Result<Pool, Error> {
    let url = format!("msyql://{}:{}@localhost:3307/{}", user, password, db_name);
    Pool::new(url.as_str())
}