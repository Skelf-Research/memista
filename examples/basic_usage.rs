//! Basic usage example of the Memista library
//!
//! This example demonstrates how to use Memista as a library in your own Rust application.
//! It shows how to create a database pool, set up application state, and start the HTTP server.

use memista::{AppState, Config, create_app};
use async_sqlite::{PoolBuilder, JournalMode};
use std::sync::Arc;
use actix_web::HttpServer;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Memista server...");
    
    // Create a configuration with default values
    let config = Config {
        database_path: "memista_example.db".to_string(),
        server_host: "127.0.0.1".to_string(),
        server_port: 8084, // Use a different port to avoid conflicts
        log_level: "info".to_string(),
    };
    
    // Create a database pool
    let db_pool = PoolBuilder::new()
        .path(&config.database_path)
        .journal_mode(JournalMode::Wal)
        .open()
        .await
        .expect("Failed to create database pool");

    // Create application state
    let app_state = Arc::new(AppState { db_pool });

    // Start the HTTP server
    let bind_address = format!("{}:{}", config.server_host, config.server_port);
    println!("Server will be available at http://{}", bind_address);
    
    // Start the server (this will block until the server is stopped)
    println!("Starting server... Press Ctrl+C to stop.");
    HttpServer::new(move || {
        create_app(app_state.clone())
    })
    .bind(bind_address)?
    .run()
    .await?;
    
    println!("Server stopped.");
    Ok(())
}