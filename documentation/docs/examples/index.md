# Examples

This section provides code examples demonstrating how to use Memista in various scenarios.

## Available Examples

The repository includes four runnable examples:

| Example | Description |
|---------|-------------|
| [Basic Usage](basic-usage.md) | Starting the HTTP server programmatically |
| [Library Usage](library-usage.md) | Direct database operations without HTTP |
| [Advanced Usage](advanced-usage.md) | Realistic document processing workflow |
| [HTTP Client](http-client.md) | Interacting with a running server via HTTP |

## Running Examples

Examples are located in the `examples/` directory. Run them with:

```bash
# Run a specific example
cargo run --example basic_usage
cargo run --example library_usage
cargo run --example advanced_usage
cargo run --example http_client
```

## Example Categories

### Server Examples

- **Basic Usage** - Demonstrates starting a Memista server with custom configuration
- Best for understanding server initialization

### Library Examples

- **Library Usage** - Shows direct database and index operations
- **Advanced Usage** - Demonstrates a complete document processing pipeline
- Best for embedding Memista in your application

### Client Examples

- **HTTP Client** - Shows HTTP API usage with reqwest
- Requires a running Memista server
- Best for integrating with Memista from any language

## Quick Reference

### Insert Data

```rust
let chunks = vec![ChunkData {
    embedding: vec![0.1, 0.2],
    text: "Sample text".to_string(),
    metadata: "{\"key\": \"value\"}".to_string(),
}];
```

### Search Data

```rust
let query = vec![0.1, 0.2];
let results = index.search(&query, 5)?;
```

### HTTP Request

```bash
curl -X POST http://localhost:8083/v1/insert \
  -H "Content-Type: application/json" \
  -d '{"database_id": "db", "chunks": [...]}'
```

## Next Steps

Browse the individual example pages for complete, working code with detailed explanations.
