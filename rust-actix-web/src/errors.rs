use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, From};


#[derive(Display, From, Debug)]
pub enum MyError {
    NotFound,
    SQLxError(sqlx::Error),
}
impl std::error::Error for MyError {}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            MyError::NotFound => HttpResponse::NotFound().finish(),
            MyError::SQLxError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
        }
    }
}
