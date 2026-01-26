# Reference

Technical reference documentation for Memista.

## Contents

- [Data Structures](data-structures.md) - Rust types and schemas
- [Error Codes](error-codes.md) - HTTP status codes and error handling

## Quick Reference

### API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/v1/insert` | POST | Insert chunks |
| `/v1/search` | POST | Search chunks |
| `/v1/drop` | DELETE | Drop database |

### Configuration

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_PATH` | `memista.db` | SQLite file path |
| `SERVER_HOST` | `127.0.0.1` | Bind address |
| `SERVER_PORT` | `8083` | Listen port |
| `LOG_LEVEL` | `info` | Logging level |

### Data Flow

```
Client Request
      │
      ▼
  HTTP API (/v1/*)
      │
      ├─────────────────────────┐
      ▼                         ▼
   SQLite                    USearch
(text, metadata)          (embeddings)
      │                         │
      └─────────────────────────┘
                  │
                  ▼
          HTTP Response
```

## Rust Crate

For programmatic usage, see the [docs.rs documentation](https://docs.rs/memista).

### Key Types

```rust
use memista::{
    AppState,           // Shared application state
    Config,             // Configuration
    ChunkData,          // Individual chunk
    InsertChunkRequest, // Insert request
    SearchRequest,      // Search request
    SearchResult,       // Search result
};
```

### Key Functions

```rust
use memista::{
    create_app,           // Create Actix-web app
    ensure_table_exists,  // Initialize database table
    load_or_create_index, // Load/create USearch index
};
```
