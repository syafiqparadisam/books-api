use actix_web::main;
use config::{DatabaseConfig, HttpConfig};
use dotenv::dotenv;
mod config;
mod handlers;
use std::env;
use crate::handlers::handlers;

#[main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("Database url is not defined")
        .parse()
        .expect("Cannot convert string to integer");
    let db_config = DatabaseConfig::new(db_url).await;

    let port = env::var("PORT")
        .expect("PORT is not defined")
        .parse()
        .expect("Cannot convert string to integer");

    handlers::new(db_config);
    let server = HttpConfig::new(port, format!("127.0.0.1"));
    let running = server.connect().await;

    running
}
