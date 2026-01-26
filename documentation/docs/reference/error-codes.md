# Error Codes

Reference for Memista error responses and HTTP status codes.

## HTTP Status Codes

### Success Codes

| Code | Status | Description |
|------|--------|-------------|
| 200 | OK | Request successful |

### Client Error Codes

| Code | Status | Description |
|------|--------|-------------|
| 400 | Bad Request | Invalid request format or parameters |
| 404 | Not Found | Database or resource not found |
| 422 | Unprocessable Entity | Valid JSON but invalid data |

### Server Error Codes

| Code | Status | Description |
|------|--------|-------------|
| 500 | Internal Server Error | Unexpected server error |

## Error Response Format

All errors return JSON with an `error` field:

```json
{
  "error": "Description of the error"
}
```

## Common Errors

### Invalid JSON

**Status:** 400 Bad Request

```json
{
  "error": "Invalid JSON: expected value at line 1 column 1"
}
```

**Cause:** Request body is not valid JSON.

**Solution:** Validate JSON before sending.

### Missing Required Field

**Status:** 400 Bad Request

```json
{
  "error": "Missing required field: database_id"
}
```

**Cause:** Required field not provided in request.

**Solution:** Include all required fields.

### Database Not Found

**Status:** 404 Not Found

```json
{
  "error": "Database 'unknown_db' not found"
}
```

**Cause:** Searching or dropping a non-existent database.

**Solution:** Verify the database_id exists.

### Invalid Embedding Dimensions

**Status:** 422 Unprocessable Entity

```json
{
  "error": "Embedding dimension mismatch: expected 2, got 3"
}
```

**Cause:** Embedding vector has wrong number of dimensions.

**Solution:** Use correct embedding dimensions.

### Empty Chunks Array

**Status:** 400 Bad Request

```json
{
  "error": "Chunks array cannot be empty"
}
```

**Cause:** Insert request with no chunks.

**Solution:** Provide at least one chunk.

### Database Connection Error

**Status:** 500 Internal Server Error

```json
{
  "error": "Database connection failed"
}
```

**Cause:** SQLite database unavailable or corrupted.

**Solution:** Check database file permissions and integrity.

### Index Error

**Status:** 500 Internal Server Error

```json
{
  "error": "Failed to load index: file not found"
}
```

**Cause:** USearch index file missing or corrupted.

**Solution:** Re-insert data or restore from backup.

## Error Handling Best Practices

### Client-Side

```rust
let response = client.post(url).json(&payload).send().await?;

match response.status() {
    StatusCode::OK => {
        let data: SuccessResponse = response.json().await?;
        // Handle success
    }
    StatusCode::BAD_REQUEST => {
        let error: ErrorResponse = response.json().await?;
        eprintln!("Bad request: {}", error.error);
    }
    StatusCode::NOT_FOUND => {
        eprintln!("Database not found");
    }
    status => {
        eprintln!("Unexpected error: {}", status);
    }
}
```

### Retry Logic

For transient errors (500), implement retry with backoff:

```rust
async fn with_retry<T, F, Fut>(f: F) -> Result<T, Error>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, Error>>,
{
    let mut attempts = 0;
    let max_attempts = 3;

    loop {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) if attempts < max_attempts => {
                attempts += 1;
                let delay = Duration::from_millis(100 * 2u64.pow(attempts));
                tokio::time::sleep(delay).await;
            }
            Err(e) => return Err(e),
        }
    }
}
```

## Debugging Errors

### Enable Debug Logging

```bash
LOG_LEVEL=debug memista
```

Debug logs include:

- Request/response details
- Database operations
- Index operations
- Timing information

### Check Server Logs

Logs indicate error source:

```
[ERROR] Database error: SQLITE_BUSY
[ERROR] Index error: dimension mismatch
[ERROR] Request error: invalid JSON
```

### Validate Requests

Use the OpenAPI spec to validate requests:

```bash
# Get the spec
curl http://localhost:8083/openapi.json > openapi.json

# Validate with a tool like openapi-spec-validator
openapi-spec-validator openapi.json
```
