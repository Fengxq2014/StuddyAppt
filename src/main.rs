use std::future::{ready, Ready};
use actix_web::{get, post, web, App, FromRequest, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use chrono::Local;
use log::{info, LevelFilter};
use serde::Serialize;
use std::io::Write;
use actix_web::dev::Payload;
use actix_web::error::ErrorBadRequest;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log_settings();
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 3000))?
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

struct WxOpenId(String);

impl FromRequest for WxOpenId {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        if let Some(user_agent) = req.headers().get("x-wx-openid") {
            if let Ok(user_agent_str) = user_agent.to_str() {
                return ready(Ok(WxOpenId(user_agent_str.to_string())));
            }
        }
        ready(Err(ErrorBadRequest("wx openid header not found")))
    }
}

#[get("/")]
async fn hello(openid: WxOpenId) -> Result<impl Responder> {
    info!("openid: {}", openid);
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
