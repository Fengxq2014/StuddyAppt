use actix_web::http::StatusCode;
use derive_more::{Display, Error, From};
use mysql::{params, Pool, PooledConn};
use mysql::prelude::Queryable;

#[derive(Debug, Display, Error, From)]
pub enum PersistenceError {
    EmptyUserName,
    EmptyPhone,
    EmptyOpenId,

    MysqlError(mysql::Error),

    Unknown,
}

impl actix_web::ResponseError for PersistenceError{
    fn status_code(&self) -> StatusCode {
        match self {
            PersistenceError::EmptyUserName
            | PersistenceError::EmptyPhone
            | PersistenceError::EmptyOpenId => StatusCode::BAD_REQUEST,

            PersistenceError::MysqlError(_) | PersistenceError::Unknown => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}
pub fn create_user (pool: &Pool, open_id: String, phone:String, name :String) -> Result<(), PersistenceError>{
    if open_id.replace(' ', "").trim().is_empty() {
        return Err(PersistenceError::EmptyOpenId);
    }
    if phone.replace(' ', "").trim().is_empty() {
        return Err(PersistenceError::EmptyPhone);
    }
    if name.replace(' ', "").trim().is_empty() {
        return Err(PersistenceError::EmptyUserName);
    }
    let mut conn = pool.get_conn()?;

    let last_insert_id = insert_user_data(&mut conn, open_id, phone, name)?;

    if last_insert_id > 0{
        Ok(())
    }else {
        Err(PersistenceError::Unknown)
    }
}

fn insert_user_data(conn: &mut PooledConn, open_id: String, phone: String, name: String) -> mysql::error::Result<u64> {
    conn.exec_drop(
        "INSERT INTO t_user VALUES (:open_id, :phone, :name)",
        params! {
            "open_id" => open_id,
            "phone" => phone,
            "name" => name
        },
    ).map(|_| {conn.last_insert_id()})
}