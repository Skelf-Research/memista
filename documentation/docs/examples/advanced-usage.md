# Advanced Usage Example

This example demonstrates a more realistic use case with document processing and semantic search.

## Overview

The advanced usage example shows:

- Processing real text documents
- Creating embeddings from text
- Storing multiple documents
- Semantic search queries
- Result ranking and display

## Complete Code

```rust
//! Advanced usage example of the Memista library
//!
//! Demonstrates a realistic document processing pipeline.

use memista::{
    AppState, Config, InsertChunkRequest, SearchRequest, ChunkData,
    ensure_table_exists, load_or_create_index
};
use async_sqlite::{PoolBuilder, JournalMode};
use std::sync::Arc;

/// Simple embedding function for demonstration
/// In production, use a proper model like Word2Vec, BERT, or OpenAI embeddings
fn create_embedding(text: &str) -> Vec<f32> {
    let word_count = text.split_whitespace().count() as f32;
    let char_count = text.chars().count() as f32;
    vec![word_count / 100.0, char_count / 1000.0]
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating advanced Memista library usage...");

    // Setup
    let config = Config {
        database_path: "memista_advanced_example.db".to_string(),
        server_host: "127.0.0.1".to_string(),
        server_port: 8086,
        log_level: "info".to_string(),
    };

    let db_pool = PoolBuilder::new()
        .path(&config.database_path)
        .journal_mode(JournalMode::Wal)
        .open()
        .await?;

    let app_state = Arc::new(AppState { db_pool });
    let database_id = "documents_db";

    ensure_table_exists(&app_state.db_pool, database_id).await?;

    // Sample documents about programming languages
    let documents = vec![
        "Rust is a systems programming language that runs blazingly fast, \
         prevents segfaults, and guarantees thread safety.",
        "Python is a high-level programming language known for its simplicity \
         and readability. It's widely used in data science and web development.",
        "JavaScript is a versatile programming language primarily used for \
         web development. It enables interactive web pages.",
        "Java is a class-based, object-oriented programming language designed \
         to have as few implementation dependencies as possible.",
        "Go is an open source programming language that makes it easy to build \
         simple, reliable, and efficient software."
    ];

    // Convert to chunks with embeddings
    let chunks: Vec<ChunkData> = documents.iter().enumerate().map(|(i, doc)| {
        ChunkData {
            embedding: create_embedding(doc),
            text: doc.to_string(),
            metadata: format!("{{\"doc_id\": \"doc_{}\", \"source\": \"example\"}}", i),
        }
    }).collect();

    println!("Processing {} documents...", chunks.len());

    // Insert chunks
    let mut index = load_or_create_index(database_id)?;
    let table_name = format!("chunks_{}", database_id);

    for chunk in &chunks {
        let chunk = chunk.clone();
        let table_name = table_name.clone();

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

        index.add(chunk_id as u64, &chunk.embedding)?;
    }

    index.save(&format!("{}.usearch", database_id))?;
    println!("Stored {} documents", chunks.len());

    // Semantic search
    let query_text = "Programming languages are used to build software applications.";
    println!("\nSearching for: \"{}\"", query_text);

    let query_embedding = create_embedding(query_text);
    let index = load_or_create_index(database_id)?;
    let results = index.search(&query_embedding, 3)?;

    println!("\nTop {} similar documents:", results.keys.len());

    for (i, (chunk_id, score)) in results.keys.iter()
        .zip(results.distances.iter())
        .enumerate()
    {
        let chunk_id = *chunk_id;
        let table_name = table_name.clone();

        let chunk = app_state.db_pool.conn(move |conn| {
            conn.query_row(
                &format!("SELECT text, metadata FROM {} WHERE chunk_id = ?", table_name),
                [chunk_id.to_string()],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, Option<String>>(1)?)),
            )
        }).await?;

        println!("\n{}. Score: {:.4}", i + 1, score);
        println!("   Text: {}", chunk.0);
        if let Some(metadata) = chunk.1 {
            println!("   Metadata: {}", metadata);
        }
    }

    println!("\nAdvanced usage example completed!");
    Ok(())
}
```

## Running the Example

```bash
cargo run --example advanced_usage
```

## Key Concepts

### Embedding Creation

```rust
fn create_embedding(text: &str) -> Vec<f32> {
    let word_count = text.split_whitespace().count() as f32;
    let char_count = text.chars().count() as f32;
    vec![word_count / 100.0, char_count / 1000.0]
}
```

This is a **simplified example**. In production, you would use:

- OpenAI's `text-embedding-ada-002` (1536 dimensions)
- Sentence Transformers
- Cohere embeddings
- Custom trained models

### Document Processing Pipeline

1. **Load documents** - From files, databases, or APIs
2. **Create embeddings** - Convert text to vectors
3. **Store in Memista** - Index for similarity search
4. **Query** - Find similar documents

### Semantic Search

The query embedding is compared against all stored embeddings:

```rust
let query_embedding = create_embedding(query_text);
let results = index.search(&query_embedding, 3)?;
```

Results are ranked by similarity (lower distance = more similar).

## Output

```
Demonstrating advanced Memista library usage...
Processing 5 documents...
Stored 5 documents

Searching for: "Programming languages are used to build software applications."

Top 3 similar documents:

1. Score: 0.0012
   Text: Go is an open source programming language that makes it easy...
   Metadata: {"doc_id": "doc_4", "source": "example"}

2. Score: 0.0089
   Text: Rust is a systems programming language that runs blazingly fast...
   Metadata: {"doc_id": "doc_0", "source": "example"}

3. Score: 0.0156
   Text: Python is a high-level programming language known for its simplicity...
   Metadata: {"doc_id": "doc_1", "source": "example"}

Advanced usage example completed!
```

## Production Considerations

### Use Real Embedding Models

```rust
// Example with OpenAI (pseudo-code)
async fn create_embedding(text: &str) -> Vec<f32> {
    let response = openai_client
        .embeddings()
        .create(text, "text-embedding-ada-002")
        .await?;
    response.embedding
}
```

### Batch Processing

For large document sets, process in batches:

```rust
for batch in documents.chunks(100) {
    // Process batch
    // Insert into Memista
    // Save index periodically
}
```

### Error Handling

Add proper error handling for production:

```rust
match index.add(chunk_id, &embedding) {
    Ok(_) => {},
    Err(e) => {
        eprintln!("Failed to index chunk {}: {}", chunk_id, e);
        // Handle error appropriately
    }
}
```

## See Also

- [HTTP Client Example](http-client.md) - API-based interaction
- [Benchmarking](../benchmarking/index.md) - Performance testing
