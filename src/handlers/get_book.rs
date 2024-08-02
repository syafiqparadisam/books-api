

use crate::config::DatabaseConfig;
use actix_web::{http::StatusCode, web, HttpResponse, Responder};

use super::ApiResponse;

pub async fn get_books(db_config: web::Data<DatabaseConfig>) -> impl Responder {
    let response: ApiResponse<String> = ApiResponse::new(StatusCode::OK, None, Some(format!("This message was ok")));
    HttpResponse::Ok().json(response)
}

pub async fn get_book_by_id(db_config: web::Data<DatabaseConfig>) -> impl Responder {
    let response: ApiResponse<String> = ApiResponse::new(StatusCode::OK, None, Some(format!("hehe")));
    HttpResponse::Ok().json(response)
}