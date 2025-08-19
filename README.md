
# Memista: High-Performance Vector Search Service

Memista is a high-performance vector search service written in Rust that provides a simple HTTP API for storing and retrieving text chunks with their associated vector embeddings. It combines SQLite for metadata storage with USearch for efficient vector similarity search.

## Features

- **Fast Vector Similarity Search**: Utilizes USearch for high-performance similarity search
- **Persistent Storage**: Stores text chunks and metadata in SQLite for durability
- **Multi-Database Support**: Supports multiple isolated databases through `database_id` partitioning
- **Comprehensive API Documentation**: Auto-generated OpenAPI documentation with Swagger, Redoc, and RapiDoc interfaces
- **Environment-Based Configuration**: Easily configurable through environment variables
- **Asynchronous I/O**: Built with async I/O for high performance and concurrency
- **Memory Efficient**: Uses optimized data structures for efficient memory usage

## Architecture

Memista uses a dual-storage approach:
- **SQLite**: Stores text chunks and their metadata with fast retrieval by ID
- **USearch**: Maintains vector embeddings for efficient similarity search

Each database is isolated with its own SQLite table and USearch index file, enabling multi-tenancy.

## API Endpoints

### POST /v1/insert
Insert text chunks with their embeddings into a specified database.

**Request Body**:
```json
{
  "database_id": "string",
  "chunks": [
    {
      "embedding": [0.1, 0.2, ...],
      "text": "string",
      "metadata": "string"
    }
  ]
}
```

### POST /v1/search
Search for similar chunks using vector embeddings.

**Request Body**:
```json
{
  "database_id": "string",
  "embeddings": [[0.1, 0.2, ...]],
  "num_results": 5
}
```

### DELETE /v1/drop
Drop a specific database and its associated vector index.

**Request Body**:
```json
{
  "database_id": "string"
}
```

## Configuration

The service can be configured using environment variables. Create a `.env` file or set environment variables directly.

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_PATH` | Path to SQLite database file | `memista.db` |
| `SERVER_HOST` | Host address to bind to | `127.0.0.1` |
| `SERVER_PORT` | Port to listen on | `8083` |
| `LOG_LEVEL` | Logging level (debug, info, warn, error) | `info` |

## Quick Start

1. Install Rust and Cargo:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone this repository:
   ```bash
   git clone <repository-url>
   cd memista
   ```

3. Create a `.env` file with your configuration (optional):
   ```bash
   cp sample.env .env
   ```

4. Check [BUILD_REQUIREMENTS.md](BUILD_REQUIREMENTS.md) for system dependencies

5. Run the server:
   ```bash
   cargo run
   ```

The server will start and the API documentation will be available at:
- Swagger UI: http://localhost:8083/swagger
- Redoc: http://localhost:8083/redoc
- RapiDoc: http://localhost:8083/rapidoc
- OpenAPI JSON: http://localhost:8083/openapi.json

## Testing

A test script is provided to verify basic functionality:

```bash
# Start Memista in one terminal
cargo run

# Run the test script in another terminal
./test_functionality.sh
```

## Example Usage

### Insert Chunks

```bash
curl -X POST http://localhost:8083/v1/insert \
  -H "Content-Type: application/json" \
  -d '{
    "database_id": "my_db",
    "chunks": [{
      "embedding": [0.1, 0.2],
      "text": "Sample text",
      "metadata": "{\"source\": \"document1\"}"
    }]
  }'
```

### Search Chunks

```bash
curl -X POST http://localhost:8083/v1/search \
  -H "Content-Type: application/json" \
  -d '{
    "database_id": "my_db",
    "embeddings": [[0.1, 0.2]],
    "num_results": 5
  }'
```

### Drop Database

```bash
curl -X DELETE http://localhost:8083/v1/drop \
  -H "Content-Type: application/json" \
  -d '{
    "database_id": "my_db"
  }'
```

## Dependencies

The project uses several key dependencies:

- **actix-web**: High-performance web framework
- **usearch**: Ultra-fast vector similarity search library
- **async-sqlite**: Async interface to SQLite
- **apistos**: OpenAPI 3.0 documentation generation
- **serde**: Serialization/deserialization framework

For a complete list of dependencies with version information, see the [Cargo.toml](Cargo.toml) file.

## Performance Considerations

1. **Embedding Dimensionality**: Currently hardcoded to 2 dimensions. For production use, this should be configurable.
2. **Index Persistence**: USearch indexes are persisted to disk for durability.
3. **Connection Pooling**: Uses connection pooling for efficient database access.
4. **Async I/O**: Fully async implementation for high concurrency.

## Benchmarking

The `benchmarks/` directory contains scripts for benchmarking Memista's performance:

- `insert_benchmark.sh`: Measures insert performance
- `search_benchmark.sh`: Measures search performance
- `concurrent_benchmark.sh`: Measures concurrent request handling
- `run_all_benchmarks.sh`: Runs all benchmarks sequentially

See the [benchmarks/README.md](benchmarks/README.md) for detailed instructions.

## Documentation

- [BUILD_REQUIREMENTS.md](BUILD_REQUIREMENTS.md): System requirements for building the project
- [UPDATES_SUMMARY.md](UPDATES_SUMMARY.md): Summary of recent codebase updates
- [benchmarks/README.md](benchmarks/README.md): Benchmarking suite documentation

## License

This project is licensed under the GNU General Public License v3.0 (GPL-3.0). See [LICENSE](LICENSE) for details.

## Contact

support@sokratis.xyz

