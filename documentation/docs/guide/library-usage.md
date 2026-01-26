# Library Usage

This guide covers using Memista as a Rust library in your own applications.

## Basic Setup

Add Memista to your `Cargo.toml`:

```toml
[dependencies]
memista = "0.1"
async-sqlite = "0.2.2"
tokio = { version = "1.38.0", features = ["full"] }
```

## Creating Application State

```rust
use memista::{AppState, Config};
use async_sqlite::{PoolBuilder, JournalMode};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration
    let config = Config {
        database_path: "memista.db".to_string(),
        server_host: "127.0.0.1".to_string(),
        server_port: 8083,
        log_level: "info".to_string(),
    };

    // Create database pool
    let db_pool = PoolBuilder::new()
        .path(&config.database_path)
        .journal_mode(JournalMode::Wal)
        .open()
        .await?;

    // Create application state
    let app_state = Arc::new(AppState { db_pool });

    Ok(())
}
```

## Direct Database Operations

### Ensuring Table Exists

Before inserting data, ensure the table exists:

```rust
use memista::ensure_table_exists;

let database_id = "my_database";
ensure_table_exists(&app_state.db_pool, database_id).await?;
```

### Loading or Creating Index

```rust
use memista::load_or_create_index;

let mut index = load_or_create_index(database_id)?;
```

### Inserting Chunks

```rust
use memista::{ChunkData, InsertChunkRequest};

// Prepare chunks
let chunks = vec![
    ChunkData {
        embedding: vec![0.1, 0.2],
        text: "First chunk".to_string(),
        metadata: "{\"source\": \"example\"}".to_string(),
    },
    ChunkData {
        embedding: vec![0.3, 0.4],
        text: "Second chunk".to_string(),
        metadata: "{\"source\": \"example\"}".to_string(),
    },
];

// Insert into database
let table_name = format!("chunks_{}", database_id);

for chunk in &chunks {
    let chunk = chunk.clone();
    let table_name = table_name.clone();

    // Insert into SQLite
    let chunk_id: i64 = app_state.db_pool.conn(move |conn| {
        conn.query_row(
            &format!(
                "INSERT INTO {} (text, metadata) VALUES (?, ?) RETURNING chunk_id",
                table_name
            ),
            [&chunk.text, &chunk.metadata],
            |row| row.get(0),
        )
    }).await?;

    // Add to vector index
    index.add(chunk_id as u64, &chunk.embedding)?;
}

// Save index to disk
let index_file = format!("{}.usearch", database_id);
index.save(&index_file)?;
```

### Searching

```rust
use memista::{SearchRequest, load_or_create_index};

// Load index
let index = load_or_create_index(database_id)?;

// Search
let query_embedding = vec![0.1, 0.2];
let num_results = 5;
let results = index.search(&query_embedding, num_results)?;

// Retrieve text from SQLite
for (chunk_id, score) in results.keys.iter().zip(results.distances.iter()) {
    let chunk_id = *chunk_id;
    let table_name = table_name.clone();

    let (text, metadata) = app_state.db_pool.conn(move |conn| {
        conn.query_row(
            &format!("SELECT text, metadata FROM {} WHERE chunk_id = ?", table_name),
            [chunk_id.to_string()],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, Option<String>>(1)?)),
        )
    }).await?;

    println!("Chunk {}: {} (score: {})", chunk_id, text, score);
}
```

## Starting the HTTP Server

You can also start the HTTP server from your library code:

```rust
use memista::{AppState, Config, create_app};
use actix_web::HttpServer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env()?;

    let db_pool = PoolBuilder::new()
        .path(&config.database_path)
        .journal_mode(JournalMode::Wal)
        .open()
        .await?;

    let app_state = Arc::new(AppState { db_pool });
    let bind_address = format!("{}:{}", config.server_host, config.server_port);

    HttpServer::new(move || {
        create_app(app_state.clone())
    })
    .bind(bind_address)?
    .run()
    .await
}
```

## Error Handling

Memista uses `anyhow` for error handling. Wrap operations in proper error handling:

```rust
use anyhow::Result;

async fn insert_data(app_state: &AppState) -> Result<()> {
    ensure_table_exists(&app_state.db_pool, "my_db").await?;
    let index = load_or_create_index("my_db")?;
    // ... operations
    Ok(())
}
```

## See Also

- [Basic Usage Example](../examples/basic-usage.md)
- [Library Usage Example](../examples/library-usage.md)
- [Advanced Usage Example](../examples/advanced-usage.md)
