mod controller;
mod core;
mod entity;
mod utils;
use core::{mysql::get_mysql_pool, thread_pool::ThreadPool};

use actix_web::{web, App, HttpServer};
use controller::task_controller::hello;
use utils::redisutil::get_redis_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let redis_pool = get_redis_pool();
    let thread_pool = ThreadPool::new(3);
    let mysql_pool = get_mysql_pool("root", "hzau123", "web").expect("无法连接数据库");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(redis_pool.clone()))
            .app_data(web::Data::new(thread_pool.clone()))
            .app_data(web::Data::new(mysql_pool.clone()))
            .service(web::scope("/model").service(hello))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
