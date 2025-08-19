use std::sync::Arc;
use actix_web::{web, App, HttpServer, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use anyhow::Result;
use usearch::{Index, IndexOptions, MetricKind, ScalarKind, new_index};
use async_sqlite::{Pool, PoolBuilder, JournalMode};
use apistos::{api_operation, ApiComponent};
use apistos::app::{BuildConfig, OpenApiWrapper};
use apistos::info::Info;
use apistos::server::Server;
use apistos::spec::Spec;
use apistos::web::{post, delete, resource, scope};
use apistos::{RapidocConfig, RedocConfig, ScalarConfig, SwaggerUIConfig};
use schemars::JsonSchema;
use dotenv::dotenv;
use std::env;

use log::{debug, info, warn};

/// Represents a chunk of text with its vector embedding and metadata
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
struct ChunkData {
    /// Vector embedding for the text chunk
    embedding: Vec<f32>,
    /// The actual text content
    text: String,
    /// Additional metadata in JSON string format
    metadata: String,
}

/// Request structure for inserting chunks into the database
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
struct InsertChunkRequest {
    /// Identifier for the database to insert into
    database_id: String,
    /// List of chunks to insert
    chunks: Vec<ChunkData>,
}

/// Request structure for searching similar chunks
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
struct SearchRequest {
    /// Identifier for the database to search in
    database_id: String,
    /// List of query embeddings to search for
    embeddings: Vec<Vec<f32>>,
    /// Maximum number of results to return per query
    num_results: usize,
}

/// Structure representing a search result
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
struct SearchResult {
    /// The text content of the matching chunk
    text: String,
    /// Optional metadata associated with the chunk
    metadata: Option<String>,
    /// Similarity score (higher means more similar)
    score: f32,
}

/// Request structure for dropping a database table and its index
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
struct DropTableRequest {
    /// Identifier for the database to drop
    database_id: String,
}

/// Application state shared across HTTP handlers
struct AppState {
    /// Database connection pool for SQLite operations
    db_pool: Pool,
}

/// Ensures that a database table exists for the given database ID
/// Creates the table if it doesn't exist with the proper schema
async fn ensure_table_exists(db_pool: &Pool, database_id: &str) -> Result<(), actix_web::Error> {
    // Table name is prefixed with "chunks_" to avoid conflicts
    let table_name = format!("chunks_{}", database_id);
    db_pool.conn(move |conn| {
        conn.execute(
            &format!("CREATE TABLE IF NOT EXISTS {} (
                chunk_id INTEGER PRIMARY KEY AUTOINCREMENT,
                text TEXT,
                metadata TEXT
            )", table_name),
            [],
        )
    }).await.map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(())
}

/// Loads an existing vector search index or creates a new one if it doesn't exist
/// The index is persisted to disk with a filename based on the database ID
fn load_or_create_index(database_id: &str) -> Result<Index, actix_web::Error> {
    // Index file name based on database ID
    let index_file = format!("{}.usearch", database_id);
    
    // Configure the index options for USearch
    let options = IndexOptions {
        // Currently hardcoded to 2 dimensions (should be configurable)
        dimensions: 2,
        // Using Inner Product metric for similarity
        metric: MetricKind::IP,
        // Using 32-bit floats for quantization
        quantization: ScalarKind::F32,
        // Default connectivity parameters
        connectivity: 0,
        expansion_add: 0,
        expansion_search: 0,
        // Enable multi-vector support
        multi: true,
    };
    
    // Create a new index with the specified options
    let index: Index = new_index(&options).map_err(actix_web::error::ErrorInternalServerError)?;
    
    // Load existing index from disk if it exists
    if std::path::Path::new(&index_file).exists() {
        index.load(&index_file).map_err(actix_web::error::ErrorInternalServerError)?;
    }
    
    Ok(index)
}

/// API endpoint for inserting chunks into the database
/// 
/// This function handles the insertion of text chunks along with their vector embeddings
/// into both the SQLite database for metadata storage and the USearch index for fast
/// vector similarity search.
#[api_operation(summary = "Insert chunks into the database")]
async fn insert_chunk(
    app_state: web::Data<Arc<AppState>>,
    request: web::Json<InsertChunkRequest>,
) -> actix_web::Result<HttpResponse> {

    log::debug!("Loading index for database: {}", &request.database_id);

    // Load or create the vector search index
    let mut index = load_or_create_index(&request.database_id)?;

    // Reserve space in the index for better performance
    index.reserve(request.chunks.len() + index.size()).map_err(actix_web::error::ErrorInternalServerError)?;

    log::debug!("Loaded index for database: {}", &request.database_id);

    // Ensure the database table exists
    ensure_table_exists(&app_state.db_pool, &request.database_id).await?;

    log::debug!("Ensured table exists for database: {}", &request.database_id);
    
    // Get the table name for this database
    let table_name = format!("chunks_{}", request.database_id);

    // Track the IDs of inserted chunks
    let mut inserted_ids = Vec::new();

    // Process each chunk in the request
    for chunk in &request.chunks {
        let chunk = chunk.clone();
        let table_name = table_name.clone();

        log::debug!("Inserting chunk into database");
        // Insert the chunk into SQLite database and get the assigned ID
        let chunk_id: i64 = app_state.db_pool.conn(move |conn| {
            conn.query_row(
                &format!("INSERT INTO {} (text, metadata) VALUES (?, ?) RETURNING chunk_id", table_name),
                [&chunk.text, &chunk.metadata],
                |row| row.get(0),
            )
        }).await.map_err(actix_web::error::ErrorInternalServerError)?;
        
        log::debug!("Inserting chunk into vector index");

        // Add the chunk's embedding to the vector search index
        index.add(chunk_id as u64, &chunk.embedding).map_err(actix_web::error::ErrorInternalServerError)?;

        // Track the inserted chunk ID
        inserted_ids.push(chunk_id);
    }

    // Save the updated index to disk
    let index_file = format!("{}.usearch", request.database_id);
    index.save(&index_file).map_err(actix_web::error::ErrorInternalServerError)?;

    // Return the IDs of inserted chunks
    Ok(HttpResponse::Ok().json(json!({ "inserted_ids": inserted_ids })))
}

/// API endpoint for searching similar chunks using vector embeddings
/// 
/// This function performs vector similarity search using the USearch index
/// and retrieves the corresponding text chunks and metadata from SQLite.
#[api_operation(summary = "Search for chunks")]
async fn search(
    app_state: web::Data<Arc<AppState>>,
    request: web::Json<SearchRequest>,
) -> actix_web::Result<HttpResponse> {
    // Load or create the vector search index
    let index = load_or_create_index(&request.database_id)?;

    // Ensure the database table exists
    ensure_table_exists(&app_state.db_pool, &request.database_id).await?;
    let table_name = format!("chunks_{}", request.database_id);

    // Store results for all query embeddings
    let mut all_results = Vec::new();

    // Process each query embedding
    for query_embedding in &request.embeddings {
        // Perform vector search using USearch
        let results = index.search(query_embedding, request.num_results).map_err(actix_web::error::ErrorInternalServerError)?;
        
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
            }).await.map_err(actix_web::error::ErrorInternalServerError)?;

            // Create a search result object
            ranked_chunks.push(SearchResult {
                text: chunk.0,
                metadata: chunk.1,
                score,
            });
        }

        // Add results for this query embedding
        all_results.push(ranked_chunks);
    }

    // Return all search results
    Ok(HttpResponse::Ok().json(all_results))
}

/// API endpoint for dropping a database table and its associated vector index
/// 
/// This function removes both the SQLite table and the USearch index file
/// for the specified database ID.
#[api_operation(summary = "Drop a table for a specific database")]
async fn drop_table(
    app_state: web::Data<Arc<AppState>>,
    request: web::Json<DropTableRequest>,
) -> actix_web::Result<HttpResponse> {
    // Get the table name for this database
    let table_name = format!("chunks_{}", request.database_id);
    
    // Drop the SQLite table if it exists
    app_state.db_pool.conn(move |conn| {
        conn.execute(
            &format!("DROP TABLE IF EXISTS {}", table_name),
            [],
        )
    }).await.map_err(actix_web::error::ErrorInternalServerError)?;
    
    // Remove the USearch index file if it exists
    let index_file = format!("{}.usearch", request.database_id);
    if std::path::Path::new(&index_file).exists() {
        std::fs::remove_file(index_file).map_err(actix_web::error::ErrorInternalServerError)?;
    }

    // Return success message
    Ok(HttpResponse::Ok().json(json!({"status": "success", "message": "Table and index dropped successfully"})))
}

/// Configuration structure for the application
#[derive(Debug, Clone)]
struct Config {
    /// Path to the SQLite database file
    database_path: String,
    /// Host address to bind the server to
    server_host: String,
    /// Port to listen on
    server_port: u16,
    /// Logging level (debug, info, warn, error)
    log_level: String,
}

impl Config {
    /// Loads configuration from environment variables
    /// Provides default values if environment variables are not set
    fn from_env() -> Result<Self, env::VarError> {
        Ok(Config {
            database_path: env::var("DATABASE_PATH").unwrap_or_else(|_| "memista.db".to_string()),
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8083".to_string())
                .parse()
                .expect("SERVER_PORT must be a number"),
            log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
        })
    }
}

/// Main entry point for the application
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
        // Configure OpenAPI specification
        let spec = Spec {
            info: Info {
                title: "Vector Search API".to_string(),
                description: Some("Vector Search API for chunk storage and retrieval".to_string()),
                ..Default::default()
            },
            servers: vec![Server {
                url: "/".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        };

        // Build the Actix-web application with API routes
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .document(spec)  // Enable OpenAPI documentation
            .service(scope("/v1")
                .service(resource("/insert").route(post().to(insert_chunk)))
                .service(resource("/search").route(post().to(search)))
                .service(resource("/drop").route(delete().to(drop_table)))
            )
            .build_with(
                "/openapi.json",
                BuildConfig::default()
                    .with(RapidocConfig::new(&"/rapidoc"))
                    .with(RedocConfig::new(&"/redoc"))
                    .with(ScalarConfig::new(&"/scalar"))
                    .with(SwaggerUIConfig::new(&"/swagger")),
            )
    })
    .bind(bind_address)?  // Bind to the configured address
    .run()  // Start the server
    .await
}
