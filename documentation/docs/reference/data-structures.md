# Data Structures

Reference documentation for Memista's data structures.

## Rust Types

### AppState

Shared application state passed to all handlers.

```rust
pub struct AppState {
    pub db_pool: Pool,
}
```

| Field | Type | Description |
|-------|------|-------------|
| `db_pool` | `Pool` | Async SQLite connection pool |

### Config

Application configuration.

```rust
pub struct Config {
    pub database_path: String,
    pub server_host: String,
    pub server_port: u16,
    pub log_level: String,
}
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `database_path` | `String` | `memista.db` | SQLite database file |
| `server_host` | `String` | `127.0.0.1` | Server bind address |
| `server_port` | `u16` | `8083` | Server port |
| `log_level` | `String` | `info` | Logging level |

#### Methods

```rust
impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, ConfigError>;
}
```

### ChunkData

Individual text chunk with embedding.

```rust
#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct ChunkData {
    pub embedding: Vec<f32>,
    pub text: String,
    pub metadata: String,
}
```

| Field | Type | Description |
|-------|------|-------------|
| `embedding` | `Vec<f32>` | Vector embedding |
| `text` | `String` | Text content |
| `metadata` | `String` | JSON metadata string |

### InsertChunkRequest

Request body for insert operations.

```rust
#[derive(Deserialize, JsonSchema)]
pub struct InsertChunkRequest {
    pub database_id: String,
    pub chunks: Vec<ChunkData>,
}
```

| Field | Type | Description |
|-------|------|-------------|
| `database_id` | `String` | Target database identifier |
| `chunks` | `Vec<ChunkData>` | Chunks to insert |

### SearchRequest

Request body for search operations.

```rust
#[derive(Deserialize, JsonSchema)]
pub struct SearchRequest {
    pub database_id: String,
    pub embeddings: Vec<Vec<f32>>,
    pub num_results: usize,
}
```

| Field | Type | Description |
|-------|------|-------------|
| `database_id` | `String` | Database to search |
| `embeddings` | `Vec<Vec<f32>>` | Query embeddings |
| `num_results` | `usize` | Max results per query |

### SearchResult

Individual search result.

```rust
#[derive(Serialize, JsonSchema)]
pub struct SearchResult {
    pub chunk_id: i64,
    pub text: String,
    pub metadata: Option<String>,
    pub score: f32,
}
```

| Field | Type | Description |
|-------|------|-------------|
| `chunk_id` | `i64` | Chunk identifier |
| `text` | `String` | Chunk text content |
| `metadata` | `Option<String>` | JSON metadata |
| `score` | `f32` | Similarity score |

### DropTableRequest

Request body for drop operations.

```rust
#[derive(Deserialize, JsonSchema)]
pub struct DropTableRequest {
    pub database_id: String,
}
```

| Field | Type | Description |
|-------|------|-------------|
| `database_id` | `String` | Database to drop |

## Database Schema

### SQLite Table Structure

Each `database_id` creates a table named `chunks_{database_id}`:

```sql
CREATE TABLE IF NOT EXISTS chunks_{database_id} (
    chunk_id INTEGER PRIMARY KEY AUTOINCREMENT,
    text TEXT NOT NULL,
    metadata TEXT
);
```

| Column | Type | Description |
|--------|------|-------------|
| `chunk_id` | `INTEGER` | Auto-incrementing primary key |
| `text` | `TEXT` | Chunk text content |
| `metadata` | `TEXT` | Optional JSON metadata |

### USearch Index

Each database has a corresponding index file: `{database_id}.usearch`

The index maps `chunk_id` (u64) to embedding vectors.

## JSON Schemas

### Insert Request

```json
{
  "type": "object",
  "required": ["database_id", "chunks"],
  "properties": {
    "database_id": { "type": "string" },
    "chunks": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["embedding", "text", "metadata"],
        "properties": {
          "embedding": {
            "type": "array",
            "items": { "type": "number" }
          },
          "text": { "type": "string" },
          "metadata": { "type": "string" }
        }
      }
    }
  }
}
```

### Search Request

```json
{
  "type": "object",
  "required": ["database_id", "embeddings", "num_results"],
  "properties": {
    "database_id": { "type": "string" },
    "embeddings": {
      "type": "array",
      "items": {
        "type": "array",
        "items": { "type": "number" }
      }
    },
    "num_results": { "type": "integer", "minimum": 1 }
  }
}
```

### Search Response

```json
{
  "type": "object",
  "properties": {
    "results": {
      "type": "array",
      "items": {
        "type": "array",
        "items": {
          "type": "object",
          "properties": {
            "chunk_id": { "type": "integer" },
            "text": { "type": "string" },
            "metadata": { "type": "string" },
            "score": { "type": "number" }
          }
        }
      }
    }
  }
}
```
