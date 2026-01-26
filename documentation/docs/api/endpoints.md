# HTTP Endpoints

Detailed documentation for all Memista HTTP endpoints.

## POST /v1/insert

Insert text chunks with their embeddings into a specified database.

### Request

```bash
curl -X POST http://localhost:8083/v1/insert \
  -H "Content-Type: application/json" \
  -d '{
    "database_id": "my_db",
    "chunks": [
      {
        "embedding": [0.1, 0.2],
        "text": "Sample text content",
        "metadata": "{\"source\": \"document1\"}"
      }
    ]
  }'
```

### Request Body

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `database_id` | string | Yes | Unique identifier for the database |
| `chunks` | array | Yes | Array of ChunkData objects |

**ChunkData object:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `embedding` | array[float] | Yes | Vector embedding |
| `text` | string | Yes | Text content |
| `metadata` | string | Yes | JSON metadata string |

### Response

```json
{
  "chunk_ids": [1, 2, 3]
}
```

| Field | Type | Description |
|-------|------|-------------|
| `chunk_ids` | array[int] | IDs assigned to inserted chunks |

---

## POST /v1/search

Search for similar chunks using vector embeddings.

### Request

```bash
curl -X POST http://localhost:8083/v1/search \
  -H "Content-Type: application/json" \
  -d '{
    "database_id": "my_db",
    "embeddings": [[0.1, 0.2]],
    "num_results": 5
  }'
```

### Request Body

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `database_id` | string | Yes | Database to search in |
| `embeddings` | array[array[float]] | Yes | Query embedding vectors |
| `num_results` | integer | Yes | Maximum results per query |

### Response

```json
{
  "results": [
    [
      {
        "chunk_id": 1,
        "text": "Sample text content",
        "metadata": "{\"source\": \"document1\"}",
        "score": 0.0
      },
      {
        "chunk_id": 2,
        "text": "Another text content",
        "metadata": "{\"source\": \"document2\"}",
        "score": 0.1234
      }
    ]
  ]
}
```

| Field | Type | Description |
|-------|------|-------------|
| `results` | array[array] | Results for each query embedding |

**SearchResult object:**

| Field | Type | Description |
|-------|------|-------------|
| `chunk_id` | integer | Chunk identifier |
| `text` | string | Text content |
| `metadata` | string | JSON metadata |
| `score` | float | Similarity score (lower = more similar) |

!!! note "Score Interpretation"
    Scores represent distance, so **lower values indicate higher similarity**. A score of 0.0 means an exact match.

---

## DELETE /v1/drop

Drop a specific database and its associated vector index.

### Request

```bash
curl -X DELETE http://localhost:8083/v1/drop \
  -H "Content-Type: application/json" \
  -d '{
    "database_id": "my_db"
  }'
```

### Request Body

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `database_id` | string | Yes | Database to drop |

### Response

```json
{
  "success": true,
  "message": "Database 'my_db' dropped successfully"
}
```

!!! warning "Destructive Operation"
    This operation permanently deletes all data in the specified database. It cannot be undone.

---

## Error Responses

### 400 Bad Request

Invalid request body or parameters:

```json
{
  "error": "Invalid request: missing required field 'database_id'"
}
```

### 404 Not Found

Database or resource not found:

```json
{
  "error": "Database 'unknown_db' not found"
}
```

### 500 Internal Server Error

Server-side error:

```json
{
  "error": "Internal server error: database connection failed"
}
```
