use actix_web::{
    main,
    web::{self},
    App, HttpServer,
};
use config::{DatabaseConfig, EnvConfig};
mod config;
mod handlers;

#[main]
async fn main() -> std::io::Result<()> {
    let env_config = EnvConfig::new();
    let db_config = DatabaseConfig::new(env_config.db_url).await;

    println!("Server is running on port {}", env_config.port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_config.clone()))
            .route("/book", web::get().to(handlers::get_book::get_books))
    })
    .bind((env_config.host.as_str(), env_config.port))?
    .run()
    .await
}
