use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{Datetime};

#[derive(Deserialize, FromRow,Debug,Serialize)]
pub struct Book {
    pub id: i32,
    pub name: String,
    pub year: String,
    pub author: String,
    pub summary: String,
    pub publisher: String,
    pub page_count: String,
    pub read_page: String,
    pub finished: bool,
    pub reading: bool,
    pub created_at: Datetime<UTC>,
    pub updated_at: Datetime<UTC>
}


