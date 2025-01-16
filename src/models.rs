use std::fmt::Display;
use std::future::{ready, Ready};
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::error::ErrorBadRequest;
use serde::{Deserialize, Serialize};

pub struct WxOpenId(String);

impl FromRequest for WxOpenId {
    type Error = actix_web::Error;
    type Future = Ready<actix_web::Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        if let Some(user_agent) = req.headers().get("x-wx-openid") {
            if let Ok(user_agent_str) = user_agent.to_str() {
                return ready(Ok(WxOpenId(user_agent_str.to_string())));
            }
        }
        ready(Err(ErrorBadRequest("wx openid header not found")))
    }
}

impl Display for WxOpenId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize)]
pub struct MyObj {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub wx_open_id: String,
    pub user_name: String,
    pub phone_number: String,
}