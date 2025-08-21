//! Basic usage example of the Memista library
//!
//! This example demonstrates how to use Memista as a library in your own Rust application.

use memista::{AppState, Config};
use async_sqlite::{PoolBuilder, JournalMode};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Memista server...");
    
    // Load configuration from environment variables or use defaults
    let config = Config::from_env().expect("Failed to load configuration");
    
    // Create a database pool
    let db_pool = PoolBuilder::new()
        .path(&config.database_path)
        .journal_mode(JournalMode::Wal)
        .open()
        .await
        .expect("Failed to create database pool");

    // Create application state
    let _app_state = Arc::new(AppState { db_pool });

    // Start the HTTP server
    let bind_address = format!("{}:{}", config.server_host, config.server_port);
    println!("Server will be available at http://{}", bind_address);
    
    // Note: In a real example, you would actually start the server.
    // For this example, we'll just show how it would be done.
    // HttpServer::new(move || {
    //     create_app(app_state.clone())
    // })
    // .bind(bind_address)?
    // .run()
    // .await?;
    
    println!("Example completed successfully!");
    Ok(())
}