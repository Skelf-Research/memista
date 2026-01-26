# HTTP Client Example

This example demonstrates how to interact with a running Memista server using Rust's HTTP client.

## Prerequisites

Before running this example, start a Memista server:

```bash
cargo run
```

## Overview

The HTTP client example shows:

- Connecting to a Memista server
- Health checking
- Inserting chunks via HTTP
- Searching via HTTP
- Dropping databases
- Error handling

## Complete Code

```rust
//! HTTP client example for Memista
//!
//! Demonstrates interacting with a running Memista server.

use reqwest;
use serde_json::{json, Value};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Demonstrating HTTP client usage with Memista...");

    let client = reqwest::Client::new();
    let base_url = "http://127.0.0.1:8083";
    let database_id = "http_client_example_db";

    // Health check
    println!("Checking if Memista server is running...");
    let health_check = client
        .get(&format!("{}/openapi.json", base_url))
        .send()
        .await;

    match health_check {
        Ok(response) if response.status().is_success() => {
            println!("Server is running");
        }
        _ => {
            println!("Server not running at {}. Start with: cargo run", base_url);
            return Ok(());
        }
    }

    // Clean up existing database
    println!("\nCleaning up existing database...");
    let _ = client
        .delete(&format!("{}/v1/drop", base_url))
        .header("Content-Type", "application/json")
        .json(&json!({"database_id": database_id}))
        .send()
        .await;

    // Insert chunks
    println!("\nInserting chunks...");
    let insert_payload = json!({
        "database_id": database_id,
        "chunks": [
            {
                "embedding": [0.1, 0.2],
                "text": "First HTTP client chunk",
                "metadata": "{\"source\": \"http_example1\"}"
            },
            {
                "embedding": [0.3, 0.4],
                "text": "Second HTTP client chunk",
                "metadata": "{\"source\": \"http_example2\"}"
            },
            {
                "embedding": [0.5, 0.6],
                "text": "Third HTTP client chunk",
                "metadata": "{\"source\": \"http_example3\"}"
            }
        ]
    });

    let insert_response = client
        .post(&format!("{}/v1/insert", base_url))
        .header("Content-Type", "application/json")
        .json(&insert_payload)
        .send()
        .await?;

    if insert_response.status().is_success() {
        let body: Value = insert_response.json().await?;
        println!("Inserted chunks: {:?}", body);
    } else {
        println!("Insert failed: {}", insert_response.status());
        return Ok(());
    }

    // Search for similar chunks
    println!("\nSearching for similar chunks...");
    let search_payload = json!({
        "database_id": database_id,
        "embeddings": [[0.1, 0.2]],
        "num_results": 5
    });

    let search_response = client
        .post(&format!("{}/v1/search", base_url))
        .header("Content-Type", "application/json")
        .json(&search_payload)
        .send()
        .await?;

    if search_response.status().is_success() {
        let body: Value = search_response.json().await?;
        println!("Search results:\n{:#}", body);
    } else {
        println!("Search failed: {}", search_response.status());
    }

    // Drop database
    println!("\nDropping database...");
    let drop_response = client
        .delete(&format!("{}/v1/drop", base_url))
        .header("Content-Type", "application/json")
        .json(&json!({"database_id": database_id}))
        .send()
        .await?;

    if drop_response.status().is_success() {
        println!("Database dropped successfully");
    }

    println!("\nHTTP client example completed!");
    Ok(())
}
```

## Running the Example

```bash
# Terminal 1: Start the server
cargo run

# Terminal 2: Run the client
cargo run --example http_client
```

## Code Walkthrough

### 1. HTTP Client Setup

```rust
let client = reqwest::Client::new();
let base_url = "http://127.0.0.1:8083";
```

Creates a reusable HTTP client instance.

### 2. Health Check

```rust
let health_check = client
    .get(&format!("{}/openapi.json", base_url))
    .send()
    .await;
```

Verifies the server is running before making requests.

### 3. Insert Request

```rust
let insert_response = client
    .post(&format!("{}/v1/insert", base_url))
    .header("Content-Type", "application/json")
    .json(&insert_payload)
    .send()
    .await?;
```

Sends a POST request with JSON body to insert chunks.

### 4. Search Request

```rust
let search_response = client
    .post(&format!("{}/v1/search", base_url))
    .header("Content-Type", "application/json")
    .json(&search_payload)
    .send()
    .await?;
```

Searches for similar chunks using a query embedding.

### 5. Error Handling

```rust
if search_response.status().is_success() {
    let body: Value = search_response.json().await?;
    println!("Results: {:#}", body);
} else {
    println!("Failed: {}", search_response.status());
}
```

Checks response status and handles errors appropriately.

## Output

```
Demonstrating HTTP client usage with Memista...
Server is running

Cleaning up existing database...

Inserting chunks...
Inserted chunks: {"chunk_ids":[1,2,3]}

Searching for similar chunks...
Search results:
{
  "results": [
    [
      {
        "chunk_id": 1,
        "text": "First HTTP client chunk",
        "metadata": "{\"source\": \"http_example1\"}",
        "score": 0.0
      },
      ...
    ]
  ]
}

Dropping database...
Database dropped successfully

HTTP client example completed!
```

## Using from Other Languages

### Python

```python
import requests

base_url = "http://localhost:8083"

# Insert
response = requests.post(f"{base_url}/v1/insert", json={
    "database_id": "python_db",
    "chunks": [{
        "embedding": [0.1, 0.2],
        "text": "Hello from Python",
        "metadata": "{}"
    }]
})
print(response.json())

# Search
response = requests.post(f"{base_url}/v1/search", json={
    "database_id": "python_db",
    "embeddings": [[0.1, 0.2]],
    "num_results": 5
})
print(response.json())
```

### JavaScript/Node.js

```javascript
const baseUrl = "http://localhost:8083";

// Insert
const insertResponse = await fetch(`${baseUrl}/v1/insert`, {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({
    database_id: "js_db",
    chunks: [{
      embedding: [0.1, 0.2],
      text: "Hello from JavaScript",
      metadata: "{}"
    }]
  })
});
console.log(await insertResponse.json());

// Search
const searchResponse = await fetch(`${baseUrl}/v1/search`, {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({
    database_id: "js_db",
    embeddings: [[0.1, 0.2]],
    num_results: 5
  })
});
console.log(await searchResponse.json());
```

## See Also

- [API Endpoints](../api/endpoints.md) - Complete API reference
- [OpenAPI Documentation](../api/openapi.md) - Interactive API docs
