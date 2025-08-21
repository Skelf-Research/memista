# Memista

[![Crates.io](https://img.shields.io/crates/v/memista.svg)](https://crates.io/crates/memista)
[![Documentation](https://docs.rs/memista/badge.svg)](https://docs.rs/memista)
[![License](https://img.shields.io/crates/l/memista.svg)](https://github.com/sokratis-xyz/memista/blob/main/LICENSE)

Memista is a high-performance vector search library that combines SQLite for metadata storage with USearch for efficient vector similarity search. It provides both a library interface for embedding in Rust applications and a standalone HTTP server.

## Features

- **Fast Vector Similarity Search**: Utilizes USearch for high-performance similarity search
- **Persistent Storage**: Stores text chunks and metadata in SQLite for durability
- **Multi-Database Support**: Supports multiple isolated databases through `database_id` partitioning
- **Comprehensive API Documentation**: Auto-generated OpenAPI documentation with Swagger, Redoc, and RapiDoc interfaces
- **Environment-Based Configuration**: Easily configurable through environment variables
- **Asynchronous I/O**: Built with async I/O for high performance and concurrency
- **Memory Efficient**: Uses optimized data structures for efficient memory usage

## Installation

### As a Library

Add this to your `Cargo.toml`:

```toml
[dependencies]
memista = "0.1"
```

### As a CLI Application

To install and run Memista as a standalone server:

```bash
# Clone the repository
git clone https://github.com/your-repo/memista.git
cd memista

# Build and run the application
cargo run

# Or build and install it
cargo build --release
./target/release/memista
```

The server will start on `http://127.0.0.1:8083` by default.

You can also install it directly from crates.io:

```bash
cargo install memista
memista
```

## Library Usage

### Basic Example

```rust
use memista::{AppState, Config, create_app, insert_chunk, search};
use async_sqlite::{PoolBuilder, JournalMode};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a database pool
    let db_pool = PoolBuilder::new()
        .path("memista.db")
        .journal_mode(JournalMode::Wal)
        .open()
        .await?;

    // Create application state
    let app_state = Arc::new(AppState { db_pool });

    // Insert some data
    let insert_request = memista::InsertChunkRequest {
        database_id: "my_database".to_string(),
        chunks: vec![memista::ChunkData {
            embedding: vec![0.1, 2.0],
            text: "Hello, world!".to_string(),
            metadata: "{\"source\": \"example\"}".to_string(),
        }],
    };

    // You would typically call the insert_chunk function through the web API
    // or integrate it directly into your application logic

    Ok(())
}
```

### Starting the HTTP Server

```rust
use memista::{AppState, Config, create_app};
use async_sqlite::{PoolBuilder, JournalMode};
use actix_web::HttpServer;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");
    
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
    HttpServer::new(move || {
        create_app(app_state.clone())
    })
    .bind(bind_address)?
    .run()
    .await
}
```

### Examples

The repository includes several examples demonstrating different ways to use Memista:

1. **Basic Usage**: Shows how to start a Memista HTTP server (`examples/basic_usage.rs`)
2. **Library Usage**: Demonstrates direct library usage without starting the HTTP server (`examples/library_usage.rs`)
3. **Advanced Usage**: Shows a more realistic use case with text processing (`examples/advanced_usage.rs`)
4. **HTTP Client**: Demonstrates how to interact with a running Memista server using Rust's HTTP client (`examples/http_client.rs`)

To run any example:

```bash
cargo run --example example_name
```

See the [examples/README.md](examples/README.md) for more details on each example.

## HTTP API

Memista provides a RESTful HTTP API for vector search operations:

### POST /v1/insert

Insert text chunks with their embeddings into a specified database.

```bash
curl -X POST http://localhost:8083/v1/insert \
  -H \"Content-Type: application/json\" \
  -d '{
    \"database_id\": \"my_db\",
    \"chunks\": [{
      \"embedding\": [0.1, 0.2],
      \"text\": \"Sample text\",
      \"metadata\": \"{\\\"source\\\": \\\"document1\\\"}\"
    }]
  }'
```

### POST /v1/search

Search for similar chunks using vector embeddings.

```bash
curl -X POST http://localhost:8083/v1/search \
  -H \"Content-Type: application/json\" \
  -d '{
    \"database_id\": \"my_db\",
    \"embeddings\": [[0.1, 0.2]],
    \"num_results\": 5
  }'
```

### DELETE /v1/drop

Drop a specific database and its associated vector index.

```bash
curl -X DELETE http://localhost:8083/v1/drop \
  -H \"Content-Type: application/json\" \
  -d '{
    \"database_id\": \"my_db\"
  }'
```

## Configuration

The service can be configured using environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_PATH` | Path to SQLite database file | `memista.db` |
| `SERVER_HOST` | Host address to bind to | `127.0.0.1` |
| `SERVER_PORT` | Port to listen on | `8083` |
| `LOG_LEVEL` | Logging level (debug, info, warn, error) | `info` |

## Documentation

- [API Documentation](https://docs.rs/memista)
- [Release Notes](UPDATES_SUMMARY.md)

## Performance Considerations

1. **Embedding Dimensionality**: Currently hardcoded to 2 dimensions. For production use, this should be configurable.
2. **Index Persistence**: USearch indexes are persisted to disk for durability.
3. **Connection Pooling**: Uses connection pooling for efficient database access.
4. **Async I/O**: Fully async implementation for high concurrency.

## License

This project is licensed under the GNU General Public License v3.0 (GPL-3.0). See [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
