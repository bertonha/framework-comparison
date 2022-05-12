use crate::{
    models::{Form, InputForm},
};
use sqlx;
use sqlx::PgPool;
use crate::errors::MyError;

pub async fn add_form(conn: &PgPool, form_info: InputForm) -> Result<Form, MyError> {
    let _sql = "INSERT INTO forms (title) VALUES ($1) RETURNING *";

    sqlx::query_as::<_, Form>(_sql)
        .bind(form_info.title)
        .fetch_one(conn)
        .await
        .map_err(|e| e.into())
}

pub async fn fetch_forms(conn: &PgPool) -> Result<Vec<Form>, MyError> {
    let _sql = "SELECT * FROM forms";

    sqlx::query_as::<_, Form>(_sql)
        .fetch_all(conn)
        .await
        .map_err(|e| e.into())
}

pub async fn fetch_form(conn: &PgPool, form_id: i64) -> Result<Form, MyError> {
    let _sql = "SELECT * FROM forms WHERE id = $1";

    sqlx::query_as::<_, Form>(_sql)
        .bind(form_id)
        .fetch_one(conn)
        .await
        .map_err(|_e| MyError::NotFound)
}
