# Request/Response Schemas

This page documents the data structures used in Memista's API.

## Request Schemas

### InsertChunkRequest

Used with `POST /v1/insert`:

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

| Field | Type | Description |
|-------|------|-------------|
| `database_id` | string | Unique identifier for the target database |
| `chunks` | ChunkData[] | Array of chunks to insert |

### ChunkData

Individual chunk within an insert request:

```json
{
  "embedding": [0.1, 0.2],
  "text": "The text content of this chunk",
  "metadata": "{\"source\": \"doc1\", \"page\": 5}"
}
```

| Field | Type | Description |
|-------|------|-------------|
| `embedding` | float[] | Vector embedding (must match configured dimensions) |
| `text` | string | Text content to store |
| `metadata` | string | JSON string containing arbitrary metadata |

!!! note "Metadata Format"
    The `metadata` field must be a valid JSON string. Common use cases include storing source information, page numbers, timestamps, or any custom attributes.

### SearchRequest

Used with `POST /v1/search`:

```json
{
  "database_id": "my_db",
  "embeddings": [[0.1, 0.2], [0.3, 0.4]],
  "num_results": 10
}
```

| Field | Type | Description |
|-------|------|-------------|
| `database_id` | string | Database to search |
| `embeddings` | float[][] | Array of query embedding vectors |
| `num_results` | integer | Maximum results to return per query |

### DropTableRequest

Used with `DELETE /v1/drop`:

```json
{
  "database_id": "my_db"
}
```

| Field | Type | Description |
|-------|------|-------------|
| `database_id` | string | Database to drop |

---

## Response Schemas

### InsertResponse

Response from `POST /v1/insert`:

```json
{
  "chunk_ids": [1, 2, 3]
}
```

| Field | Type | Description |
|-------|------|-------------|
| `chunk_ids` | integer[] | Auto-generated IDs for inserted chunks |

### SearchResponse

Response from `POST /v1/search`:

```json
{
  "results": [
    [
      {
        "chunk_id": 1,
        "text": "First matching chunk",
        "metadata": "{\"source\": \"doc1\"}",
        "score": 0.0
      },
      {
        "chunk_id": 5,
        "text": "Second matching chunk",
        "metadata": "{\"source\": \"doc2\"}",
        "score": 0.234
      }
    ]
  ]
}
```

| Field | Type | Description |
|-------|------|-------------|
| `results` | SearchResult[][] | Nested array: outer for each query, inner for results |

### SearchResult

Individual search result:

| Field | Type | Description |
|-------|------|-------------|
| `chunk_id` | integer | Unique identifier of the chunk |
| `text` | string | Text content of the chunk |
| `metadata` | string | JSON metadata string |
| `score` | float | Distance score (lower = more similar) |

### DropResponse

Response from `DELETE /v1/drop`:

```json
{
  "success": true,
  "message": "Database 'my_db' dropped successfully"
}
```

| Field | Type | Description |
|-------|------|-------------|
| `success` | boolean | Whether the operation succeeded |
| `message` | string | Human-readable status message |

---

## Error Response

All endpoints may return error responses:

```json
{
  "error": "Description of what went wrong"
}
```

| Field | Type | Description |
|-------|------|-------------|
| `error` | string | Error description |

---

## Embedding Dimensions

!!! important "Embedding Dimensionality"
    Currently, Memista is configured for 2-dimensional embeddings. All embeddings must have exactly 2 elements. This is suitable for demonstration purposes.

    For production use with real embedding models (e.g., OpenAI's text-embedding-ada-002 with 1536 dimensions), the dimensionality configuration would need to be updated.

## Metadata Best Practices

The metadata field accepts any valid JSON string. Recommended patterns:

```json
// Source tracking
"{\"source\": \"document.pdf\", \"page\": 42}"

// Timestamps
"{\"created_at\": \"2025-01-15T10:30:00Z\"}"

// Custom attributes
"{\"category\": \"technical\", \"language\": \"en\", \"author\": \"John\"}"

// Nested structures
"{\"document\": {\"id\": \"abc123\", \"version\": 2}}"
```
