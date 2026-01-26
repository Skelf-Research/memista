# Basic Usage Example

This example demonstrates how to start a Memista HTTP server programmatically.

## Overview

The basic usage example shows:

- Creating configuration
- Setting up a database pool
- Initializing application state
- Starting the HTTP server

## Complete Code

```rust
//! Basic usage example of the Memista library
//!
//! This example demonstrates how to use Memista as a library
//! in your own Rust application.

use memista::{AppState, Config, create_app};
use async_sqlite::{PoolBuilder, JournalMode};
use std::sync::Arc;
use actix_web::HttpServer;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Memista server...");

    // Create a configuration with custom values
    let config = Config {
        database_path: "memista_example.db".to_string(),
        server_host: "127.0.0.1".to_string(),
        server_port: 8084, // Different port to avoid conflicts
        log_level: "info".to_string(),
    };

    // Create a database pool with WAL journal mode
    let db_pool = PoolBuilder::new()
        .path(&config.database_path)
        .journal_mode(JournalMode::Wal)
        .open()
        .await
        .expect("Failed to create database pool");

    // Create application state (shared across all handlers)
    let app_state = Arc::new(AppState { db_pool });

    // Prepare the bind address
    let bind_address = format!("{}:{}", config.server_host, config.server_port);
    println!("Server will be available at http://{}", bind_address);

    // Start the HTTP server
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
```

## Running the Example

```bash
cargo run --example basic_usage
```

## Code Walkthrough

### 1. Configuration

```rust
let config = Config {
    database_path: "memista_example.db".to_string(),
    server_host: "127.0.0.1".to_string(),
    server_port: 8084,
    log_level: "info".to_string(),
};
```

Creates configuration with:

- Custom database file path
- Localhost binding
- Port 8084 (to avoid conflicts with default 8083)
- Info-level logging

### 2. Database Pool

```rust
let db_pool = PoolBuilder::new()
    .path(&config.database_path)
    .journal_mode(JournalMode::Wal)
    .open()
    .await
    .expect("Failed to create database pool");
```

Creates an async SQLite connection pool with:

- WAL (Write-Ahead Logging) mode for better concurrency
- Automatic connection management

### 3. Application State

```rust
let app_state = Arc::new(AppState { db_pool });
```

Wraps the database pool in `Arc` for thread-safe sharing across HTTP handlers.

### 4. HTTP Server

```rust
HttpServer::new(move || {
    create_app(app_state.clone())
})
.bind(bind_address)?
.run()
.await?;
```

Starts the Actix-web server with:

- Memista's pre-configured application routes
- The shared application state
- Async runtime integration

## Testing

Once running, test with curl:

```bash
# Health check
curl http://localhost:8084/openapi.json

# Insert a chunk
curl -X POST http://localhost:8084/v1/insert \
  -H "Content-Type: application/json" \
  -d '{
    "database_id": "test",
    "chunks": [{
      "embedding": [0.1, 0.2],
      "text": "Hello from basic example",
      "metadata": "{}"
    }]
  }'
```

## See Also

- [Library Usage](library-usage.md) - Direct database operations
- [Configuration Guide](../guide/configuration.md) - Configuration options
