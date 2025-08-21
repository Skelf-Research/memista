//! Advanced usage example of the Memista library
//!
//! This example demonstrates a more realistic use case where we:
//! - Process actual text documents
//! - Create embeddings (using a simple approach for this example)
//! - Store and search for similar documents
//! - Clean up resources when done

use memista::{AppState, Config, InsertChunkRequest, SearchRequest, ChunkData, ensure_table_exists, load_or_create_index};
use async_sqlite::{PoolBuilder, JournalMode};
use std::sync::Arc;

/// A simple function to create embeddings from text
/// In a real application, you would use a proper embedding model
fn create_embedding(text: &str) -> Vec<f32> {
    // This is a very simple example - in practice you would use a model like Word2Vec, BERT, etc.
    // For demonstration purposes, we'll create a simple embedding based on text properties
    let word_count = text.split_whitespace().count() as f32;
    let char_count = text.chars().count() as f32;
    vec![word_count / 100.0, char_count / 1000.0] // Normalize to keep values small
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating advanced Memista library usage...");
    
    // Create a configuration with default values
    let config = Config {
        database_path: "memista_advanced_example.db".to_string(),
        server_host: "127.0.0.1".to_string(),
        server_port: 8086,
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
    
    // Define a database ID for our example
    let database_id = "documents_db";
    
    // Ensure the table exists for this database
    ensure_table_exists(&app_state.db_pool, database_id).await?;
    
    // Sample documents to process
    let documents = vec![
        "Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.",
        "Python is a high-level programming language known for its simplicity and readability. It's widely used in data science and web development.",
        "JavaScript is a versatile programming language primarily used for web development. It enables interactive web pages and is an essential part of web applications.",
        "Java is a class-based, object-oriented programming language that is designed to have as few implementation dependencies as possible.",
        "Go is an open source programming language that makes it easy to build simple, reliable, and efficient software."
    ];
    
    // Convert documents to chunks with embeddings
    let chunks: Vec<ChunkData> = documents.iter().enumerate().map(|(i, doc)| {
        ChunkData {
            embedding: create_embedding(doc),
            text: doc.to_string(),
            metadata: format!("{{\"doc_id\": \"doc_{}\", \"source\": \"example\"}}", i),
        }
    }).collect();
    
    println!("Processing {} documents...", chunks.len());
    
    // Load or create the vector search index
    let mut index = load_or_create_index(database_id)?;
    
    // Create an insert request
    let insert_request = InsertChunkRequest {
        database_id: database_id.to_string(),
        chunks: chunks.clone(),
    };
    
    // Get the table name for this database
    let table_name = format!("chunks_{}", database_id);
    
    // Track the IDs of inserted chunks
    let mut inserted_ids = Vec::new();
    
    // Process each chunk in the request
    for chunk in &insert_request.chunks {
        let chunk = chunk.clone();
        let table_name = table_name.clone();
        
        // Insert the chunk into SQLite database and get the assigned ID
        let chunk_id: i64 = app_state.db_pool.conn(move |conn| {
            conn.query_row(
                &format!("INSERT INTO {} (text, metadata) VALUES (?, ?) RETURNING chunk_id", table_name),
                [&chunk.text, &chunk.metadata],
                |row| row.get(0),
            )
        }).await?;
        
        // Add the chunk's embedding to the vector search index
        index.add(chunk_id as u64, &chunk.embedding)?;
        
        // Track the inserted chunk ID
        inserted_ids.push(chunk_id);
    }
    
    // Save the updated index to disk
    let index_file = format!("{}.usearch", database_id);
    index.save(&index_file)?;
    
    println!("Successfully processed and stored {} documents", chunks.len());
    
    // Now let's search for similar documents
    println!("\nSearching for documents similar to: \"Programming languages are used to build software applications.\"");
    
    // Create a query embedding
    let query_text = "Programming languages are used to build software applications.";
    let query_embedding = create_embedding(query_text);
    
    // Create a search request
    let search_request = SearchRequest {
        database_id: database_id.to_string(),
        embeddings: vec![query_embedding],
        num_results: 3, // Get the top 3 most similar documents
    };
    
    // Load the vector search index
    let index = load_or_create_index(&search_request.database_id)?;
    
    // Process the query embedding
    let query_embedding = &search_request.embeddings[0];
    
    // Perform vector search using USearch
    let results = index.search(query_embedding, search_request.num_results)?;
    
    // Collect and display the search results
    println!("\nTop {} similar documents:", results.keys.len());
    for (i, (chunk_id, score)) in results.keys.iter().zip(results.distances.iter()).enumerate() {
        let chunk_id = *chunk_id;
        let score = *score;
        let table_name = table_name.clone();
        
        // Retrieve the text and metadata from SQLite
        let chunk = app_state.db_pool.conn(move |conn| {
            conn.query_row(
                &format!("SELECT text, metadata FROM {} WHERE chunk_id = ?", table_name),
                [chunk_id.to_string()],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, Option<String>>(1)?)),
            )
        }).await?;
        
        println!("\n{}. Similarity Score: {:.4}", i + 1, score);
        println!("   Text: {}", chunk.0);
        if let Some(metadata) = chunk.1 {
            println!("   Metadata: {}", metadata);
        }
    }
    
    println!("\nAdvanced usage example completed successfully!");
    
    Ok(())
}