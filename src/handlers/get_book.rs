use crate::{
    repository::book::{Book, Repository},
};
use actix_web::{http::StatusCode, web, HttpResponse, Responder};

use super::ApiResponse;

pub async fn get_books(repo: web::Data<Repository>) -> impl Responder {
    let books = repo.find_all().await.unwrap();
    let response: ApiResponse<Vec<Book>> = ApiResponse::new(
        StatusCode::OK,
        Some(books),
        Some(format!("This message was ok")),
    );
    HttpResponse::Ok().json(response)
}

pub async fn get_book_by_id(_repo: web::Data<Repository>) -> impl Responder {
    let response: ApiResponse<String> =
        ApiResponse::new(StatusCode::OK, None, Some(format!("hehe")));
    HttpResponse::Ok().json(response)
}
