mod controller;
mod core;
mod entity;
mod utils;
use core::{connection::get_connection, thread_pool::ThreadPool};
use actix_web::{web, App, HttpServer};
use controller::task_controller::hello;
use utils::redisutil::get_redis_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut redis_pool = get_redis_pool();
    let mut thread_pool = ThreadPool::new(3);
    // let data_source = DataSource::new("root", "123456", "mysql", "localhost", 3306, "web");
    let mut mysql_conn = get_connection().await.expect("数据库连接失败");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(redis_pool.clone()))
            .app_data(web::Data::new(thread_pool.clone()))
            .app_data(web::Data::new(mysql_conn.clone()))
            .service(web::scope("/model").service(hello))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
