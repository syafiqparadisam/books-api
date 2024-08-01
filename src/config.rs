use std::{io, sync::Arc};
use tokio::sync::Notify;

use crate::handlers::get_book::get_books;
use actix_web::{App, HttpServer};
use tokio::signal;



#[derive(Debug)]
pub struct DatabaseConfig {
    url: String,
}

#[derive(Debug)]
pub struct HttpConfig {
    port: i32,
    host: String,
    shutdown_notify: Arc<Notify>,
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
    pub fn new(port: i32, host: String) -> HttpConfig {
        HttpConfig {
            port,
            host,
            shutdown_notify: Arc::new(Notify::new()),
        }
    }

    pub async fn connect(&self) -> io::Result<()> {
        let shutdown_notify = self.shutdown_notify.clone();

        let server = HttpServer::new(move || App::new().service(get_books))
            .bind((self.host.as_str(), self.port))?
            .run();

        // Tunggu notifikasi shutdown
        let shutdown = async {
            shutdown_notify.notified().await;
        };
        // Setup signal handling for graceful shutdown
        let shutdown_notify_clone = shutdown_notify.clone();
        let signal_handler = async {
            signal::ctrl_c()
                .await
                .expect("Failed to install Ctrl+C signal handler");
            println!("Received shutdown signal");
            shutdown_notify_clone.notify_one();
        };

        // Run server and signal handler concurrently
        tokio::try_join!(server, signal_handler)?;
    }

    pub fn shutdown(&self) {
        self.shutdown_notify.notify_one();
    }
}
