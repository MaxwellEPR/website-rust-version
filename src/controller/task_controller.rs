use crate::core::mail::send_mail;
use crate::core::query::Query;
use crate::core::template::get_template_as_string;
use crate::core::thread_pool::ThreadPool;
use crate::core::validation::seq_validate::evalidate_task_id;
use crate::entity::task_body::{TaskBody, TaskStatus};
use crate::utils::csvutils::{read_all_heatmap, read_by_page, read_csv_with_header, read_heatmap};
use crate::utils::fileutil::read_dir_item;
use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpResponse, Responder};
use futures_util::StreamExt;
use r2d2_redis::r2d2::Pool as RedisPool;
use r2d2_redis::redis::Commands;
use r2d2_redis::RedisConnectionManager;
use sea_orm::DatabaseConnection;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::process::{Command, Stdio};
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
                        let res = read_csv_with_header(Path::new(&ele), 1, 10000).unwrap();
                        let strres = serde_json::to_string(&res).unwrap();
                        resp.insert(ele, strres);
                    } else {
                        let res = read_heatmap(path, 0).unwrap();
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

#[post("/submit/custom")]
async fn submit(
    mut task_body: web::Json<TaskBody>,
    redis: web::Data<RedisPool<RedisConnectionManager>>,
    mysql_conn: web::Data<DatabaseConnection>,
    thread_pool: web::Data<ThreadPool>,
) -> impl Responder {
    task_body.status = TaskStatus::Submit;
    if let Ok(redis_conn) = redis.get().as_mut() {
        redis_conn.set::<&String, String, String>(&task_body.task_id, task_body.to_string());
    } else {
        return HttpResponse::InternalServerError();
    }

    let model = Query::query_by_name(&task_body.model_name, mysql_conn.as_ref())
        .await
        .unwrap();
    if model.model_name != task_body.model_name {
        return HttpResponse::Conflict();
    }

    thread_pool.execute(move || {
        task_body.status = TaskStatus::Processing;
        let mut redis_conn: r2d2_redis::r2d2::PooledConnection<RedisConnectionManager> =
            redis.get().unwrap();
        redis_conn.set::<&String, String, String>(&task_body.task_id, task_body.to_string());
        let exec_path = model.pypath.split(";").collect::<Vec<&str>>();
        let codes = model.pycode.split(";").collect::<Vec<&str>>();

        for i in 0..exec_path.len() {
            let mut child = Command::new(exec_path[i])
                .arg(codes[i])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();

            let mut s = String::from(&task_body.task_id);
            s.push(';');
            s.push_str(&model.model_name);
            child.stdin.take().unwrap().write_all(s.as_bytes());

            let mut output = String::new();
            child.stdout.take().unwrap().read_to_string(&mut output);

            let mut variable = HashMap::new();
            variable.insert("taskID", task_body.task_id.to_owned());
            variable.insert("status", task_body.status.to_string());
            variable.insert("from", task_body.email.to_string());
            if let Ok(exit_code) = child.wait() {
                if exit_code.success() {
                    task_body.status = TaskStatus::Complete;
                } else {
                    task_body.status = TaskStatus::Fail;
                }
            } else {
                task_body.status = TaskStatus::Fail;
            }
            redis_conn.set::<&String, String, String>(&task_body.task_id, task_body.to_string());

            if let Ok(template) = get_template_as_string(&variable) {
                send_mail(&task_body.email, template);
            } else {
            }
        }
    });

    HttpResponse::Ok()
}

#[post("/submit/file")]
pub async fn submit_file(mut payload: Multipart) -> impl Responder {
    let mut task_body = TaskBody::new();
    while let Some(item) = payload.next().await {
        if let Ok(mut field) = item {
            if let Some(file_name) = field.content_disposition().get_filename() {
                let file_path = format!("upload/{}", file_name);
                let mut file = File::create(file_path).unwrap();

                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    file.write(&data);
                }
            } else {
            };
        }
    }

    let file = fs::read_to_string(format!("result/{}.fasta", task_body.task_id)).unwrap();
    let lines = file.lines().collect::<Vec<&str>>();

    let mut i = 0;
    let mut seqs = vec![];
    while i + 1 < lines.len() {
        let s = format!("{}\n{}", lines.get(i).unwrap(), lines.get(i + 1).unwrap());
        seqs.push(s);
        i = i + 2;
    }


    HttpResponse::Ok()
}

#[get("/result/predict")]
pub async fn get_predict(
    task_id: web::Query<String>,
    page: web::Query<usize>,
    limit: web::Query<usize>,
    redis: web::Data<RedisPool<RedisConnectionManager>>,
) -> HttpResponse {
    if !evalidate_task_id(&task_id) || page.0.lt(&0) || limit.0.le(&0) {
        return HttpResponse::BadRequest().into();
    }

    match redis.get() {
        Ok(mut conn) => {
            if let Ok(task_body) = conn.get::<String, TaskBody>(task_id.0) {
                if let Ok(result) = read_by_page(&task_body, page.0, limit.0) {
                    return HttpResponse::Ok().json(result);
                };
            } else {
                return HttpResponse::BadRequest().into();
            }
        }
        Err(_) => {
            return HttpResponse::BadRequest().into();
        }
    };

    HttpResponse::Ok().into()
}

#[get("/result/saliency")]
pub async fn get_saliency(
    task_id: web::Query<String>,
    id: web::Query<usize>,
    redis: web::Data<RedisPool<RedisConnectionManager>>,
) -> HttpResponse {
    if !evalidate_task_id(&task_id) || id.0.lt(&0) {
        return HttpResponse::BadRequest().into();
    }

    if let Ok(mut conn) = redis.get() {
        if let Ok(task_body) = conn.get::<&String, TaskBody>(&task_id.0) {
            if let Ok(result) = read_all_heatmap(&task_body, id.0) {
                return HttpResponse::Ok().body(serde_json::to_string(&result).unwrap());
            };
        };
    }

    HttpResponse::BadRequest().into()
}

#[cfg(test)]
mod test {}
