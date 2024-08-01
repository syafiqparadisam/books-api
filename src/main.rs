use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
      App::new().route("/", web::get().to(HttpResponse::Ok))
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}

