# HTTP Server

This guide covers running Memista as a standalone HTTP server.

## Starting the Server

### From Installed Binary

```bash
memista
```

### From Source

```bash
cargo run
```

### With Custom Configuration

```bash
DATABASE_PATH=/data/memista.db SERVER_PORT=9000 memista
```

## Server Output

When started, you'll see:

```
[INFO] Starting Memista server on 127.0.0.1:8083
[INFO] Database path: memista.db
```

## API Documentation

Memista automatically generates OpenAPI documentation accessible at:

| Interface | URL |
|-----------|-----|
| OpenAPI JSON | `http://localhost:8083/openapi.json` |
| Swagger UI | `http://localhost:8083/swagger-ui/` |
| Redoc | `http://localhost:8083/redoc/` |
| RapiDoc | `http://localhost:8083/rapidoc/` |
| Scalar | `http://localhost:8083/scalar/` |

## Health Check

Verify the server is running:

```bash
curl http://localhost:8083/openapi.json
```

A successful response indicates the server is healthy.

## Production Deployment

### Systemd Service

Create `/etc/systemd/system/memista.service`:

```ini
[Unit]
Description=Memista Vector Search Service
After=network.target

[Service]
Type=simple
User=memista
WorkingDirectory=/opt/memista
Environment=DATABASE_PATH=/var/lib/memista/memista.db
Environment=SERVER_HOST=0.0.0.0
Environment=SERVER_PORT=8083
Environment=LOG_LEVEL=warn
ExecStart=/opt/memista/memista
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl enable memista
sudo systemctl start memista
```

### Reverse Proxy with Nginx

```nginx
upstream memista {
    server 127.0.0.1:8083;
}

server {
    listen 80;
    server_name memista.example.com;

    location / {
        proxy_pass http://memista;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### Docker

Create a `Dockerfile`:

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/memista /usr/local/bin/
EXPOSE 8083
CMD ["memista"]
```

Build and run:

```bash
docker build -t memista .
docker run -p 8083:8083 -v memista-data:/data \
  -e DATABASE_PATH=/data/memista.db \
  -e SERVER_HOST=0.0.0.0 \
  memista
```

## Logging

### Log Levels

Set via `LOG_LEVEL` environment variable:

- `debug` - All messages including detailed operation info
- `info` - Standard operational messages (default)
- `warn` - Warnings and errors only
- `error` - Errors only

### Log Output

Logs are written to stderr. Redirect as needed:

```bash
memista 2>&1 | tee /var/log/memista.log
```

## Performance Tuning

### Connection Pooling

Memista uses connection pooling for SQLite. The pool is managed automatically.

### Index Persistence

USearch indexes are persisted to disk. The index file is named `{database_id}.usearch`.

### Async I/O

All operations are asynchronous, allowing high concurrency. The server can handle many simultaneous requests efficiently.

## See Also

- [Configuration](configuration.md) - All configuration options
- [API Endpoints](../api/endpoints.md) - HTTP API reference
- [Benchmarking](../benchmarking/index.md) - Performance testing
