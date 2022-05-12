use crate::{db, models::InputForm};
use actix_web::{get, post, web, Error, HttpResponse};
use sqlx::PgPool;

#[post("/forms")]
pub async fn create_form(
    input_form: web::Json<InputForm>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    let new_form = db::add_form(&db_pool.into_inner(), input_form.into_inner()).await?;

    Ok(HttpResponse::Ok().json(new_form))
}

#[get("/forms")]
pub async fn get_forms(db_pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let forms = db::fetch_forms(&db_pool.into_inner()).await?;

    Ok(HttpResponse::Ok().json(forms))
}

#[get("/forms/{form_id}")]
pub async fn get_form(
    form_id: web::Path<i64>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    let form = db::fetch_form(&db_pool.into_inner(), form_id.into_inner()).await?;

    Ok(HttpResponse::Ok().json(form))
}
