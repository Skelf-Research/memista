//! Library usage example of the Memista library
//!
//! This example demonstrates how to use Memista's core functionality directly
//! without starting the HTTP server. It shows how to:
//! - Create a database pool
//! - Insert chunks directly using the library functions
//! - Search for similar chunks
//! - Clean up resources

use memista::{AppState, Config, InsertChunkRequest, SearchRequest, ChunkData, ensure_table_exists, load_or_create_index};
use async_sqlite::{PoolBuilder, JournalMode};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating Memista library usage...");
    
    // Create a configuration with default values
    let config = Config {
        database_path: "memista_library_example.db".to_string(),
        server_host: "127.0.0.1".to_string(),
        server_port: 8085,
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
    let database_id = "example_db";
    
    // Ensure the table exists for this database
    ensure_table_exists(&app_state.db_pool, database_id).await?;
    
    // Load or create the vector search index
    let mut index = load_or_create_index(database_id)?;
    
    // Prepare data to insert
    let chunks = vec![
        ChunkData {
            embedding: vec![0.1, 0.2],
            text: "This is the first example chunk".to_string(),
            metadata: "{\"source\": \"example1\"}".to_string(),
        },
        ChunkData {
            embedding: vec![0.3, 0.4],
            text: "This is the second example chunk".to_string(),
            metadata: "{\"source\": \"example2\"}".to_string(),
        },
        ChunkData {
            embedding: vec![0.5, 0.6],
            text: "This is the third example chunk".to_string(),
            metadata: "{\"source\": \"example3\"}".to_string(),
        },
    ];
    
    // Create an insert request
    let insert_request = InsertChunkRequest {
        database_id: database_id.to_string(),
        chunks: chunks.clone(),
    };
    
    println!("Inserting {} chunks into the database...", chunks.len());
    
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
    
    println!("Successfully inserted chunks with IDs: {:?}", inserted_ids);
    
    // Now let's search for similar chunks
    println!("\nSearching for similar chunks...");
    
    // Create a search request
    let search_request = SearchRequest {
        database_id: database_id.to_string(),
        embeddings: vec![vec![0.1, 0.2]], // Search for chunks similar to this embedding
        num_results: 5, // Get up to 5 results
    };
    
    // Load the vector search index
    let index = load_or_create_index(&search_request.database_id)?;
    
    // Process the query embedding
    let query_embedding = &search_request.embeddings[0];
    
    // Perform vector search using USearch
    let results = index.search(query_embedding, search_request.num_results)?;
    
    // Collect the search results with text and metadata
    let mut ranked_chunks = Vec::new();
    for (chunk_id, score) in results.keys.iter().zip(results.distances.iter()) {
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
        
        println!("Found chunk: '{}' with score: {} and metadata: {:?}", chunk.0, score, chunk.1);
        ranked_chunks.push((chunk.0, chunk.1, score));
    }
    
    println!("\nLibrary usage example completed successfully!");
    println!("Found {} similar chunks", ranked_chunks.len());
    
    Ok(())
}