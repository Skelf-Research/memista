# Configuration

Memista is configured through environment variables. You can set these directly or use a `.env` file.

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_PATH` | Path to SQLite database file | `memista.db` |
| `SERVER_HOST` | Host address to bind to | `127.0.0.1` |
| `SERVER_PORT` | Port to listen on | `8083` |
| `LOG_LEVEL` | Logging level | `info` |

## Log Levels

Available log levels (from most to least verbose):

- `debug` - Detailed debugging information
- `info` - General operational information
- `warn` - Warning messages
- `error` - Error messages only

## Using a .env File

Create a `.env` file in your project root:

```bash
# Memista Configuration
DATABASE_PATH=memista.db
SERVER_HOST=127.0.0.1
SERVER_PORT=8083
LOG_LEVEL=info
```

A sample configuration file is provided at `config/sample.env`.

## Configuration Examples

### Development Setup

```bash
DATABASE_PATH=dev_memista.db
SERVER_HOST=127.0.0.1
SERVER_PORT=8083
LOG_LEVEL=debug
```

### Production Setup

```bash
DATABASE_PATH=/var/lib/memista/memista.db
SERVER_HOST=0.0.0.0
SERVER_PORT=8083
LOG_LEVEL=warn
```

### Testing Setup

```bash
DATABASE_PATH=test_memista.db
SERVER_HOST=127.0.0.1
SERVER_PORT=8084
LOG_LEVEL=debug
```

## Programmatic Configuration

When using Memista as a library, you can create a `Config` struct directly:

```rust
use memista::Config;

let config = Config {
    database_path: "my_database.db".to_string(),
    server_host: "127.0.0.1".to_string(),
    server_port: 8083,
    log_level: "info".to_string(),
};
```

Or load from environment:

```rust
use memista::Config;

let config = Config::from_env()
    .expect("Failed to load configuration");
```

## Database Path Considerations

The `DATABASE_PATH` determines where Memista stores data:

- **Relative paths** are relative to the working directory
- **Absolute paths** are recommended for production
- Ensure the directory exists and is writable
- USearch index files are stored alongside the database (e.g., `mydb.usearch`)

## Network Configuration

### Binding to localhost only (default)

```bash
SERVER_HOST=127.0.0.1
```

Only accepts connections from the local machine.

### Binding to all interfaces

```bash
SERVER_HOST=0.0.0.0
```

!!! warning
    Binding to all interfaces exposes the service to the network. Ensure proper firewall rules and authentication if needed.
