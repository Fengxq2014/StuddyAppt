use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use chrono::Local;
use log::{info, LevelFilter};
use serde::Serialize;
use std::io::Write;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log_settings();
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

fn log_settings() {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::builder()
        .format(|buf, record| {
            // 自定义日志格式
            writeln!(
                buf,
                "[{}] [{}] {}:{} - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S %3f"), // 时间戳
                record.level(), // 日志级别
                record.module_path().unwrap_or("<unknown>"), // 模块路径
                record.line().unwrap_or(0), // 行号
                record.args() // 日志消息
            )
        })
        .filter(None, LevelFilter::Info) // 设置默认日志级别
        .init();
}

#[derive(Serialize)]
struct MyObj {
    name: String,
}
#[get("/")]
async fn hello() -> Result<impl Responder> {
    info!("hello");
    let obj = MyObj {
        name: String::from("Hello World"),
    };
    Ok(web::Json(obj))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
