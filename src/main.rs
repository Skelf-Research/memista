//! Memista HTTP server binary
//!
//! This is the main entry point for the Memista HTTP server.
//! It sets up the application with configuration from environment variables
//! and starts the Actix-web server.

use std::sync::Arc;
use actix_web::HttpServer;
use memista::{AppState, Config, create_app};
use async_sqlite::{PoolBuilder, JournalMode};
use dotenv::dotenv;

use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Load configuration from environment variables
    let config = Config::from_env().expect("Failed to load configuration");
    
    // Initialize logger with configured level
    std::env::set_var("RUST_LOG", &config.log_level);
    env_logger::init();
    
    // Create a connection pool for the SQLite database
    let db_pool = PoolBuilder::new()
        .path(&config.database_path)
        .journal_mode(JournalMode::Wal)  // Use Write-Ahead Logging for better concurrency
        .open()
        .await
        .expect("Failed to create database pool");

    // Create the shared application state
    let app_state = Arc::new(AppState {
        db_pool,
    });

    // Format the bind address for the HTTP server
    let bind_address = format!("{}:{}", config.server_host, config.server_port);
    
    // Log server startup information
    info!("Starting server on {}", bind_address);
    
    // Create and configure the HTTP server
    HttpServer::new(move || {
        create_app(app_state.clone())
    })
    .bind(bind_address)?  // Bind to the configured address
    .run()  // Start the server
    .await
}