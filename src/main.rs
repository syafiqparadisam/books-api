use actix_web::{
    main,
    web::{self},
    App, HttpServer,
};
use config::{DatabaseConfig, EnvConfig};
mod config;
use sqlx::{self, PgPool};
mod handlers;
mod repository;

#[main]
async fn main() -> std::io::Result<()> {
    let env_config = EnvConfig::new();
    let db_config = DatabaseConfig::new(env_config.db_url).await;

    sqlx::migrate!("./migrations").run(&db_config.pool).await.unwrap();

    let repo = repository::book::Repository::new(db_config);

    println!("Server is running on port {}", env_config.port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(repo.clone()))
            .route("/book", web::get().to(handlers::get_book::get_books))
    })
    .bind((env_config.host.as_str(), env_config.port))?
    .run()
    .await
}
