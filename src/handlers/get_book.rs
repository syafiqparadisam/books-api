use actix_web::{get, HttpResponse, Responder};

use crate::config::DatabaseConfig;

use super::handlers;

impl handlers {
    pub fn new(db_config: DatabaseConfig) -> handlers {
        handlers { db_config }
    }

    pub async fn get_books() -> impl Responder {
        return HttpResponse::Ok();
    }
}
