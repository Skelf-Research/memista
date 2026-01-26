# Quick Start

Get Memista up and running in 5 minutes.

## 1. Start the Server

```bash
cargo install memista
memista
```

You should see output indicating the server is running on `http://127.0.0.1:8083`.

## 2. Insert Your First Chunk

Insert a text chunk with an embedding vector:

```bash
curl -X POST http://localhost:8083/v1/insert \
  -H "Content-Type: application/json" \
  -d '{
    "database_id": "quickstart_db",
    "chunks": [{
      "embedding": [0.1, 0.2],
      "text": "Hello, this is my first chunk!",
      "metadata": "{\"source\": \"quickstart\"}"
    }]
  }'
```

**Response:**

```json
{"chunk_ids": [1]}
```

## 3. Add More Chunks

Insert multiple chunks at once:

```bash
curl -X POST http://localhost:8083/v1/insert \
  -H "Content-Type: application/json" \
  -d '{
    "database_id": "quickstart_db",
    "chunks": [
      {
        "embedding": [0.3, 0.4],
        "text": "Second chunk with different embedding",
        "metadata": "{\"source\": \"quickstart\"}"
      },
      {
        "embedding": [0.15, 0.25],
        "text": "Third chunk similar to the first",
        "metadata": "{\"source\": \"quickstart\"}"
      }
    ]
  }'
```

## 4. Search for Similar Chunks

Find chunks similar to a query embedding:

```bash
curl -X POST http://localhost:8083/v1/search \
  -H "Content-Type: application/json" \
  -d '{
    "database_id": "quickstart_db",
    "embeddings": [[0.1, 0.2]],
    "num_results": 5
  }'
```

**Response:**

```json
{
  "results": [
    [
      {
        "chunk_id": 1,
        "text": "Hello, this is my first chunk!",
        "metadata": "{\"source\": \"quickstart\"}",
        "score": 0.0
      },
      {
        "chunk_id": 3,
        "text": "Third chunk similar to the first",
        "metadata": "{\"source\": \"quickstart\"}",
        "score": 0.0707
      }
    ]
  ]
}
```

Lower scores indicate higher similarity (distance-based).

## 5. Clean Up

Drop the database when done:

```bash
curl -X DELETE http://localhost:8083/v1/drop \
  -H "Content-Type: application/json" \
  -d '{
    "database_id": "quickstart_db"
  }'
```

## Next Steps

- [API Endpoints](../api/endpoints.md) - Full API documentation
- [Configuration](../guide/configuration.md) - Customize settings
- [Examples](../examples/index.md) - More usage examples
