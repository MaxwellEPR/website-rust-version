mod csvutils;
mod dateutil;
mod entity;
mod fileutil;
mod redisutil;
use csvutils::{readCSVWithHeader, readHeatMap};
use fileutil::read_dir_item;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

#[get("/status")]
async fn hello() -> impl Responder {
    // fs::read_dir("E:/rust/webiste");
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
                        let res = readCSVWithHeader(Path::new(&ele)).unwrap();
                        let strres = serde_json::to_string(&res).unwrap();
                        resp.insert(ele, strres);
                    } else {
                        let res = readHeatMap(Path::new(&ele), 0).unwrap();
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

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::scope("/model").service(hello)))
        .bind(("localhost", 8080))?
        .run()
        .await
}
