# Memista

[![Crates.io](https://img.shields.io/crates/v/memista.svg)](https://crates.io/crates/memista)
[![Documentation](https://docs.rs/memista/badge.svg)](https://docs.rs/memista)
[![License](https://img.shields.io/crates/l/memista.svg)](https://github.com/sokratis-xyz/memista/blob/main/LICENSE)

Memista is a high-performance vector search library that combines SQLite for metadata storage with USearch for efficient vector similarity search. It provides both a library interface for embedding in Rust applications and a standalone HTTP server.

## Features

- **Fast Vector Similarity Search** - Utilizes USearch for high-performance similarity search
- **Persistent Storage** - Stores text chunks and metadata in SQLite for durability
- **Multi-Database Support** - Supports multiple isolated databases through `database_id` partitioning
- **Comprehensive API Documentation** - Auto-generated OpenAPI documentation with Swagger, Redoc, and RapiDoc interfaces
- **Environment-Based Configuration** - Easily configurable through environment variables
- **Asynchronous I/O** - Built with async I/O for high performance and concurrency
- **Memory Efficient** - Uses optimized data structures for efficient memory usage

## Quick Start

### Install as a Library

Add this to your `Cargo.toml`:

```toml
[dependencies]
memista = "0.1"
```

### Install as a CLI Application

```bash
cargo install memista
memista
```

The server will start on `http://127.0.0.1:8083` by default.

## Basic Usage

Insert a text chunk with its embedding:

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

Search for similar chunks:

```bash
curl -X POST http://localhost:8083/v1/search \
  -H "Content-Type: application/json" \
  -d '{
    "database_id": "my_db",
    "embeddings": [[0.1, 0.2]],
    "num_results": 5
  }'
```

## Next Steps

- [Installation Guide](getting-started/installation.md) - Detailed installation instructions
- [Quick Start Tutorial](getting-started/quickstart.md) - Get up and running in 5 minutes
- [API Reference](api/endpoints.md) - Complete HTTP API documentation
- [Examples](examples/index.md) - Code examples for common use cases
