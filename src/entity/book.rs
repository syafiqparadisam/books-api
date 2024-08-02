use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow};
use chrono::{Utc, Datetime};
use uuid::Uuid;

trait Repository {
    fn find_all() -> Result<Vec<Book>, Error>;
    fn find_by_id(id: i64) -> Result<Book,Error>;
}
#[derive(Deserialize, FromRow,Debug,Serialize)]
pub struct Book {
    pub id: Uuid,
    pub name: String,
    pub year: String,
    pub author: String,
    pub summary: String,
    pub publisher: String,
    pub page_count: String,
    pub read_page: String,
    pub finished: bool,
    pub reading: bool,
    pub created_at: Datetime<Utc>,
    pub updated_at: Datetime<Utc>
}


impl Repository for Book {
   
    pub fn find_all() -> Result<Vec<Book>, Error> {
        
        let books = sqlx::query!().execute()
    }
}


