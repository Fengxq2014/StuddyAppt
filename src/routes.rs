use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web::cookie::time::macros::date;
use log::info;
use crate::models::{MyObj, WxOpenId};
use crate::persistence::create_user;

#[get("/")]
pub(crate) async fn hello(openid: WxOpenId, data: web::Data<mysql::Pool>) -> actix_web::Result<impl Responder> {
    info!("openid: {}", openid);
    info!("hello");
    web::block(move || create_user(&data, openid.to_string(), String::from("13333333333"), String::from("test"))).await??;

    let obj = MyObj {
        name: String::from("Hello World"),
    };
    Ok(web::Json(obj))
}

#[post("/echo")]
pub(crate) async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub(crate) async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}