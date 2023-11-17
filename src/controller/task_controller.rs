use crate::core::query::Query;
use crate::core::thread_pool::ThreadPool;
use crate::entity::task_body::{TaskBody, TaskStatus};
use crate::utils::csvutils::{read_as_heatmap, read_csv_with_header};
use crate::utils::fileutil::read_dir_item;
use actix_web::{get, post, web, HttpResponse, Responder};
use r2d2_redis::r2d2::{Error, Pool as RedisPool};
use r2d2_redis::redis::Commands;
use r2d2_redis::RedisConnectionManager;
use sea_orm::DatabaseConnection;
use std::{collections::HashMap, path::Path};

#[get("/status")]
async fn hello(pool: web::Data<RedisPool<RedisConnectionManager>>) -> impl Responder {
    // fs::read_dir("E:/rust/webiste");
    let mut conn = pool.get().unwrap();
    let redis_result = conn.keys::<&str, Vec<TaskBody>>("*_*").unwrap();
    if let Ok(res) = serde_json::to_string(&redis_result) {
        return HttpResponse::Ok().body(res);
    } else {
        return HttpResponse::InternalServerError().body("服务器异常");
    }
}

#[get("/captcha")]
async fn captcha() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/show/{type}")]
async fn echo(path_var: web::Path<String>) -> impl Responder {
    let mut resp: HashMap<String, String> = HashMap::new();
    match path_var.as_str() {
        "maize" | "cotton" | "rice" | "wheat" => {
            let spath = format!("/home/website/data/{}", path_var.as_str());
            let path = Path::new(spath.as_str());
            if let Ok(files) = read_dir_item(path, |s| s.ends_with(".csv")) {
                for ele in files {
                    if ele.starts_with("t") {
                        let res = read_csv_with_header(Path::new(&ele)).unwrap();
                        let strres = serde_json::to_string(&res).unwrap();
                        resp.insert(ele, strres);
                    } else {
                        let res = read_as_heatmap(Path::new(&ele), 0).unwrap();
                        let strres = serde_json::to_string(&res).unwrap();
                        resp.insert(ele, strres);
                    }
                }
            } else {
                return HttpResponse::InternalServerError();
            }
        }
        _ => {
            return HttpResponse::InternalServerError();
        }
    }
    HttpResponse::Ok()
}

#[post("/submit")]
async fn submit(
    mut task_body: web::Json<TaskBody>,
    redis_manager: web::Data<RedisPool<RedisConnectionManager>>,
    mysql_conn: web::Data<DatabaseConnection>,
    thread_pool: web::Data<ThreadPool>,
) -> impl Responder {
    let model = Query::query_by_name(&task_body.model_name, mysql_conn.as_ref())
        .await
        .unwrap();

    if model.model_name != task_body.model_name {
        return HttpResponse::Conflict();
    }

    let mut r_manager = redis_manager.get();
    let redis_conn = r_manager.as_mut().unwrap();
    task_body.status = TaskStatus::Processing;
    redis_conn.set::<&String, String, String>(&task_body.task_id, task_body.to_string());

    thread_pool.execute(|| {});

    HttpResponse::Ok()
}

#[cfg(test)]
mod test {
    use std::process::{Command, Stdio};

    #[test]
    pub fn test_command() {
        
    }
}
