use dotenv::dotenv;
use std::env;

#[derive(Debug,Clone)]
pub struct DatabaseConfig {
    url: String,
}

#[derive(Debug)]
pub struct HttpConfig {
    port: u16,
    host: String,
}

#[derive(Debug)]
pub struct EnvConfig {
    pub db_url: String,
    pub port: u16,
    pub host: String
}

impl EnvConfig {
    pub fn new() -> EnvConfig {
        dotenv().ok();

        let db_url: String = env::var("POSTGRES_URL")
            .expect("Postgres url is not defined");          

        let port: u16 = env::var("APP_PORT")
            .expect("PORT is not defined")
            .parse()
            .expect("Cannot convert string to integer");

        let host: String = env::var("APP_HOST")
            .expect("Host is not defined");
        
        EnvConfig { db_url, port, host }
    }
}

impl DatabaseConfig {
    pub async fn new(url: String) -> DatabaseConfig {
        sqlx::PgPool::connect(&url)
            .await
            .expect("Failed to connect to database");
        DatabaseConfig { url }
    }
}

impl HttpConfig {
    pub fn new(port: u16, host: String) -> HttpConfig {
        HttpConfig { port, host }
    }
}
