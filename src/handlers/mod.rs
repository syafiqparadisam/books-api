use serde::{Deserialize, Serialize};
use actix_web::http::StatusCode;
pub mod get_book;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    status_code: u16,
    data: Option<T>,
    message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn new(status_code: StatusCode, data: Option<T>, message: Option<String>) -> ApiResponse<T> {
        ApiResponse {
            status_code: status_code.as_u16(),
            data: data,
            message: message,
        }
    }
}
