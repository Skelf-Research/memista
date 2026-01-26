# Library Usage Example

This example demonstrates how to use Memista's core functionality directly without starting the HTTP server.

## Overview

The library usage example shows:

- Direct database operations
- Creating and managing vector indexes
- Inserting chunks programmatically
- Searching for similar chunks
- Working with results

## Complete Code

```rust
//! Library usage example of the Memista library
//!
//! Demonstrates direct library usage without the HTTP server.

use memista::{
    AppState, Config, InsertChunkRequest, SearchRequest, ChunkData,
    ensure_table_exists, load_or_create_index
};
use async_sqlite::{PoolBuilder, JournalMode};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating Memista library usage...");

    // Configuration
    let config = Config {
        database_path: "memista_library_example.db".to_string(),
        server_host: "127.0.0.1".to_string(),
        server_port: 8085,
        log_level: "info".to_string(),
    };

    // Create database pool
    let db_pool = PoolBuilder::new()
        .path(&config.database_path)
        .journal_mode(JournalMode::Wal)
        .open()
        .await
        .expect("Failed to create database pool");

    let app_state = Arc::new(AppState { db_pool });
    let database_id = "example_db";

    // Ensure table exists
    ensure_table_exists(&app_state.db_pool, database_id).await?;

    // Load or create vector index
    let mut index = load_or_create_index(database_id)?;

    // Prepare chunks to insert
    let chunks = vec![
        ChunkData {
            embedding: vec![0.1, 0.2],
            text: "First example chunk".to_string(),
            metadata: "{\"source\": \"example1\"}".to_string(),
        },
        ChunkData {
            embedding: vec![0.3, 0.4],
            text: "Second example chunk".to_string(),
            metadata: "{\"source\": \"example2\"}".to_string(),
        },
        ChunkData {
            embedding: vec![0.5, 0.6],
            text: "Third example chunk".to_string(),
            metadata: "{\"source\": \"example3\"}".to_string(),
        },
    ];

    // Insert chunks
    let table_name = format!("chunks_{}", database_id);
    let mut inserted_ids = Vec::new();

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
        inserted_ids.push(chunk_id);
    }

    // Save index to disk
    let index_file = format!("{}.usearch", database_id);
    index.save(&index_file)?;

    println!("Inserted chunks with IDs: {:?}", inserted_ids);

    // Search for similar chunks
    println!("\nSearching for similar chunks...");

    let index = load_or_create_index(database_id)?;
    let query_embedding = vec![0.1, 0.2];
    let results = index.search(&query_embedding, 5)?;

    // Display results
    for (chunk_id, score) in results.keys.iter().zip(results.distances.iter()) {
        let chunk_id = *chunk_id;
        let table_name = table_name.clone();

        let chunk = app_state.db_pool.conn(move |conn| {
            conn.query_row(
                &format!("SELECT text, metadata FROM {} WHERE chunk_id = ?", table_name),
                [chunk_id.to_string()],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, Option<String>>(1)?)),
            )
        }).await?;

        println!("Chunk: '{}' (score: {}, metadata: {:?})", chunk.0, score, chunk.1);
    }

    println!("\nLibrary usage example completed!");
    Ok(())
}
```

## Running the Example

```bash
cargo run --example library_usage
```

## Code Walkthrough

### 1. Table Initialization

```rust
ensure_table_exists(&app_state.db_pool, database_id).await?;
```

Creates the SQLite table if it doesn't exist. Each `database_id` gets its own table.

### 2. Index Management

```rust
let mut index = load_or_create_index(database_id)?;
```

Loads an existing USearch index from disk or creates a new one.

### 3. Inserting Chunks

```rust
// Insert into SQLite (returns auto-generated ID)
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

// Add embedding to vector index
index.add(chunk_id as u64, &chunk.embedding)?;
```

Chunks are stored in two places:

- **SQLite**: Text and metadata (with auto-generated ID)
- **USearch**: Embedding vector (keyed by chunk ID)

### 4. Persisting the Index

```rust
index.save(&index_file)?;
```

Saves the vector index to disk for persistence across restarts.

### 5. Searching

```rust
let results = index.search(&query_embedding, 5)?;
```

Returns chunk IDs and distances for the most similar vectors.

### 6. Retrieving Text

```rust
let chunk = app_state.db_pool.conn(move |conn| {
    conn.query_row(
        &format!("SELECT text, metadata FROM {} WHERE chunk_id = ?", table_name),
        [chunk_id.to_string()],
        |row| Ok((row.get::<_, String>(0)?, row.get::<_, Option<String>>(1)?)),
    )
}).await?;
```

Fetches the actual text content from SQLite using the chunk ID.

## Output

```
Demonstrating Memista library usage...
Inserted chunks with IDs: [1, 2, 3]

Searching for similar chunks...
Chunk: 'First example chunk' (score: 0.0, metadata: Some("{\"source\": \"example1\"}"))
Chunk: 'Second example chunk' (score: 0.1, metadata: Some("{\"source\": \"example2\"}"))
Chunk: 'Third example chunk' (score: 0.2, metadata: Some("{\"source\": \"example3\"}"))

Library usage example completed!
```

## See Also

- [Advanced Usage](advanced-usage.md) - More realistic document processing
- [Library Usage Guide](../guide/library-usage.md) - Detailed library documentation
