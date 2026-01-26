# OpenAPI Documentation

Memista automatically generates OpenAPI (Swagger) documentation for its HTTP API.

## Accessing Documentation

When the server is running, access interactive documentation at:

| Interface | URL | Description |
|-----------|-----|-------------|
| Swagger UI | `/swagger-ui/` | Interactive API explorer |
| Redoc | `/redoc/` | Clean, responsive documentation |
| RapiDoc | `/rapidoc/` | Modern documentation viewer |
| Scalar | `/scalar/` | Developer-friendly API docs |
| OpenAPI JSON | `/openapi.json` | Raw specification |

## Using Swagger UI

Swagger UI provides an interactive interface to explore and test the API:

1. Navigate to `http://localhost:8083/swagger-ui/`
2. Browse available endpoints
3. Click an endpoint to expand it
4. Click "Try it out" to test
5. Fill in parameters and click "Execute"
6. View the response

### Example: Testing Insert

1. Expand `POST /v1/insert`
2. Click "Try it out"
3. Enter the request body:
   ```json
   {
     "database_id": "test_db",
     "chunks": [{
       "embedding": [0.1, 0.2],
       "text": "Test chunk",
       "metadata": "{\"source\": \"swagger\"}"
     }]
   }
   ```
4. Click "Execute"
5. View the response below

## Using Redoc

Redoc provides a clean, searchable documentation interface:

1. Navigate to `http://localhost:8083/redoc/`
2. Use the sidebar to navigate endpoints
3. View request/response schemas
4. Copy code examples

## Using RapiDoc

RapiDoc offers a modern, customizable documentation experience:

1. Navigate to `http://localhost:8083/rapidoc/`
2. Switch between light/dark themes
3. Use the search functionality
4. Test endpoints directly

## Using the Raw OpenAPI Spec

Fetch the raw specification for tooling integration:

```bash
curl http://localhost:8083/openapi.json
```

### Importing into Postman

1. Open Postman
2. Click "Import"
3. Select "Link"
4. Enter `http://localhost:8083/openapi.json`
5. Click "Import"

### Importing into Insomnia

1. Open Insomnia
2. Click "Import/Export"
3. Select "Import Data"
4. Choose "From URL"
5. Enter `http://localhost:8083/openapi.json`

## Code Generation

Use the OpenAPI spec to generate client libraries:

### Python Client (using openapi-generator)

```bash
openapi-generator-cli generate \
  -i http://localhost:8083/openapi.json \
  -g python \
  -o ./memista-client-python
```

### TypeScript Client

```bash
openapi-generator-cli generate \
  -i http://localhost:8083/openapi.json \
  -g typescript-fetch \
  -o ./memista-client-ts
```

### Go Client

```bash
openapi-generator-cli generate \
  -i http://localhost:8083/openapi.json \
  -g go \
  -o ./memista-client-go
```

## Schema Documentation

The OpenAPI specification includes full schema documentation for:

- Request bodies
- Response formats
- Data types
- Required fields
- Example values

This enables IDE autocompletion and validation when using generated clients.
