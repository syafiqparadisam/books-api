use crate::config::DatabaseConfig;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Debug, Serialize,Clone)]
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct Repository {
    pg_pool: DatabaseConfig,
}

impl Repository {
    pub fn new(pool: DatabaseConfig) -> Self {
        Self { pg_pool: pool }
    }
    pub async fn find_all(&self) -> Result<Vec<Book>, sqlx::Error> {
        let books = sqlx::query_as("SELECT * FROM book").fetch_all(&self.pg_pool.pool).await?;
        Ok(books)
    }
}
