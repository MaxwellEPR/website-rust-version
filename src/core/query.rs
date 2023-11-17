use crate::entity::model_entity::{self, Entity as ExecEnvCode, Model};
use actix_web::web::block;
use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter, DatabaseConnection};

use super::connection::get_connection;
pub struct Query;

impl Query {
    pub async fn query_by_name(name: &String,mysql_conn:&DatabaseConnection) -> Result<Model, DbErr> {
        Ok(ExecEnvCode::find()
                    .filter(model_entity::Column::Name.eq(name))
                    .one(mysql_conn).await?.unwrap())
    }    
}

#[cfg(test)]
mod test {
    use std::future::Future;

    use crate::{entity::model_entity::Model, core::connection::get_connection};

    use super::Query;

    #[actix_web::test]
    pub async fn test_query_by_name() {
        let conn = get_connection().await.expect("数据库连接错误");
        // let result = Query::query_by_name(String::from("aaa"),conn).await.unwrap();
        // assert_eq!(result.len(),0);
    }
}
