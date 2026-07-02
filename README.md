# memista — vector search for Rust

<p align="center">
  <h1 align="center">Memista</h1>
  <p align="center">A lightweight vector search library for Rust</p>
</p>

<p align="center">
  <a href="https://crates.io/crates/memista"><img src="https://img.shields.io/crates/v/memista.svg" alt="Crates.io"></a>
  <a href="https://docs.rs/memista"><img src="https://docs.rs/memista/badge.svg" alt="Documentation"></a>
  <a href="https://github.com/Skelf-Research/memista/blob/main/LICENSE"><img src="https://img.shields.io/crates/l/memista.svg" alt="License"></a>
  <img src="https://img.shields.io/badge/rust-1.56+-orange.svg" alt="Rust Version">
  <img src="https://img.shields.io/badge/status-experimental-yellow.svg" alt="Status">
</p>

<p align="center">
  <a href="https://memista.skelfresearch.com"><b>Website</b></a> •
  <a href="https://docs.skelfresearch.com/memista">Documentation</a> •
  <a href="https://skelfresearch.com">Skelf Research</a>
</p>

---

Memista combines **SQLite** for metadata storage with **USearch** for vector similarity search. It's designed for developers who need a simple, self-contained vector search solution without the complexity of dedicated vector databases.

## Why Memista?

- **Zero infrastructure** - Single binary, no external services required
- **Familiar storage** - SQLite for metadata means easy debugging and backups
- **Simple API** - REST endpoints that just work
- **Rust library** - Embed directly in your application
- **Multi-tenant ready** - Isolated databases via `database_id`

> **Note**: Memista is experimental and best suited for prototypes, small-to-medium datasets, and applications where simplicity matters more than scale.

## Quick Start

### Install & Run

```bash
cargo install memista
memista
```

Server starts at `http://127.0.0.1:8083`

### Store a vector

```bash
curl -X POST http://localhost:8083/v1/insert \
  -H "Content-Type: application/json" \
  -d '{
    "database_id": "my_app",
    "chunks": [{
      "embedding": [0.1, 0.2],
      "text": "Hello world",
      "metadata": "{\"source\": \"readme\"}"
    }]
  }'
```

### Search for similar vectors

```bash
curl -X POST http://localhost:8083/v1/search \
  -H "Content-Type: application/json" \
  -d '{
    "database_id": "my_app",
    "embeddings": [[0.1, 0.2]],
    "num_results": 5
  }'
```

## Use as a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
memista = "0.1"
```

```rust
use memista::{AppState, ChunkData, InsertChunkRequest};
use async_sqlite::{PoolBuilder, JournalMode};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_pool = PoolBuilder::new()
        .path("memista.db")
        .journal_mode(JournalMode::Wal)
        .open()
        .await?;

    let app_state = Arc::new(AppState { db_pool });

    // Now use app_state with Memista's functions
    Ok(())
}
```

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/v1/insert` | POST | Store chunks with embeddings |
| `/v1/search` | POST | Find similar chunks |
| `/v1/drop` | DELETE | Remove a database |
| `/openapi.json` | GET | OpenAPI specification |
| `/swagger-ui/` | GET | Interactive API docs |

## Configuration

```bash
# Environment variables (or use .env file)
DATABASE_PATH=memista.db    # SQLite file location
SERVER_HOST=127.0.0.1       # Bind address
SERVER_PORT=8083            # Listen port
LOG_LEVEL=info              # debug|info|warn|error
```

## Project Structure

```
memista/
├── src/                    # Core library and server
├── examples/               # Usage examples
├── benchmarks/             # Performance scripts
├── documentation/          # MkDocs site
├── scripts/                # Dev utilities
└── config/                 # Sample configurations
```

## Examples

```bash
cargo run --example basic_usage      # Start server programmatically
cargo run --example library_usage    # Direct library usage
cargo run --example advanced_usage   # Document processing pipeline
cargo run --example http_client      # HTTP client demo
```

## Documentation

**Local docs:**
```bash
cd documentation && pip install -r requirements.txt && mkdocs serve
```

- [User Guide](documentation/docs/index.md) - Getting started & examples
- [API Reference](https://docs.rs/memista) - Rust documentation

## Limitations

Be aware of these constraints:

- **Embedding dimensions**: Currently hardcoded to 2D (demo purposes)
- **Scale**: Not tested beyond ~100k vectors
- **Persistence**: Index rebuilds on dimension changes
- **No auth**: Add a reverse proxy for production

## Contributing

Contributions welcome! This is an experimental project, so:

1. Open an issue first for major changes
2. Keep PRs focused and small
3. Add tests for new functionality

## License

[GPL-3.0](LICENSE) - Free to use, modify, and distribute with same license.

---

<p align="center">
  <sub>Built with SQLite + USearch + Actix-web</sub>
</p>

---

## Part of Skelf Research

`memista` is built by **[Skelf Research](https://skelfresearch.com)** — an independent UK AI research lab publishing production-grade open-source projects.

🌐 [Website](https://memista.skelfresearch.com) · 📚 [Documentation](https://docs.skelfresearch.com/memista) · 🔬 [All projects](https://skelfresearch.com/projects) · 🤗 [Hugging Face](https://huggingface.co/skelfresearch)

**Related projects:** [embedcache](https://embedcache.skelfresearch.com) (local embedding cache) · [polymathy](https://polymathy.skelfresearch.com) (answer-engine service) · [slorg](https://slorg.skelfresearch.com) (search that thinks first)

<sub>Released under MIT / Apache-2.0. © Skelf Research Limited.</sub>
