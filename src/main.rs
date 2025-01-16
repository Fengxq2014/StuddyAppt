mod models;
mod persistence;
mod routes;

use actix_web::dev::Payload;
use actix_web::error::ErrorBadRequest;
use actix_web::web::Data;
use actix_web::{
    get, post, web, App, FromRequest, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use chrono::Local;
use log::{info, LevelFilter};
use mysql::Pool;
use serde::Serialize;
use std::env;
use std::fmt::Display;
use std::future::{ready, Ready};
use std::io::Write;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log_settings();
    let conn = get_conn();
    HttpServer::new(move || {
        App::new()
            .app_data(conn.clone())
            .service(routes::hello)
            .service(routes::echo)
            .route("/hey", web::get().to(routes::manual_hello))
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}

fn get_conn() -> Data<Pool> {
    dotenvy::dotenv().ok();
    let db_user = env::var("MYSQL_USER").unwrap_or(String::from("root"));
    let db_password = env::var("MYSQL_PASSWORD").unwrap_or(String::from("root"));
    let db_host = env::var("MYSQL_HOST").unwrap_or(String::from("127.0.0.1"));
    let db_port = env::var("MYSQL_PORT").unwrap_or(String::from("3306"));
    let db_name = env::var("MYSQL_DBNAME").unwrap_or(String::from("test"));
    let db_port = db_port.parse().unwrap();
    let builder = get_conn_builder(db_user, db_password, db_host, db_port, db_name);
    info!("initializing database connection");
    let pool = mysql::Pool::new(builder).unwrap();

    Data::new(pool)
}

fn get_conn_builder(
    db_user: String,
    db_password: String,
    db_host: String,
    db_port: u16,
    db_name: String,
) -> mysql::OptsBuilder {
    mysql::OptsBuilder::new()
        .ip_or_hostname(Some(db_host))
        .tcp_port(db_port)
        .db_name(Some(db_name))
        .user(Some(db_user))
        .pass(Some(db_password))
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
                record.level(),                               // 日志级别
                record.module_path().unwrap_or("<unknown>"),  // 模块路径
                record.line().unwrap_or(0),                   // 行号
                record.args()                                 // 日志消息
            )
        })
        .filter(None, LevelFilter::Info) // 设置默认日志级别
        .init();
}
