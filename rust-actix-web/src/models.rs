use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize)]
pub struct Form {
    pub id: i64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct InputForm {
    pub title: String,
}
