# API Reference

Memista provides a RESTful HTTP API for vector search operations.

## Base URL

```
http://localhost:8083
```

## API Version

All endpoints are prefixed with `/v1/`.

## Endpoints Overview

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/v1/insert` | POST | Insert text chunks with embeddings |
| `/v1/search` | POST | Search for similar chunks |
| `/v1/drop` | DELETE | Drop a database and its index |

## Content Type

All requests must include:

```
Content-Type: application/json
```

## Response Format

Successful responses return JSON with HTTP 200 status code.

Error responses return JSON with an appropriate error status code:

```json
{
  "error": "Error description"
}
```

## OpenAPI Documentation

Interactive API documentation is available at:

- **Swagger UI**: `http://localhost:8083/swagger-ui/`
- **Redoc**: `http://localhost:8083/redoc/`
- **RapiDoc**: `http://localhost:8083/rapidoc/`
- **Scalar**: `http://localhost:8083/scalar/`

Raw OpenAPI specification:

- **JSON**: `http://localhost:8083/openapi.json`

## Guide

- [HTTP Endpoints](endpoints.md) - Detailed endpoint documentation
- [Request/Response](request-response.md) - Data schemas
- [OpenAPI](openapi.md) - Using the interactive documentation
