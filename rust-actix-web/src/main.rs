mod config;
mod db;
mod errors;
mod handlers;
mod models;

use crate::handlers::{create_form, get_form, get_forms};
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = config::Config::from_env().unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(config.database_url.as_str())
        .await
        .expect("Unable to form database pool");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Unable execute migration");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(create_form)
            .service(get_forms)
            .service(get_form)
    })
    .bind(config.server_addr)?
    .run()
    .await
}
