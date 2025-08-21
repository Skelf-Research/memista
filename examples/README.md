# Memista Examples

This directory contains examples demonstrating how to use the Memista library in different ways.

## Basic Usage

The `basic_usage.rs` example shows how to start a Memista HTTP server:

```bash
cargo run --example basic_usage
```

This will start a server on `http://127.0.0.1:8084` that you can interact with using the HTTP API.

## Library Usage

The `library_usage.rs` example shows how to use Memista's core functionality directly without starting the HTTP server:

```bash
cargo run --example library_usage
```

This example demonstrates:
- Creating a database pool
- Inserting chunks directly using the library functions
- Searching for similar chunks
- Cleaning up resources

## Advanced Usage

The `advanced_usage.rs` example shows a more realistic use case with actual text processing:

```bash
cargo run --example advanced_usage
```

This example demonstrates:
- Processing actual text documents
- Creating simple embeddings (in a real application, you would use a proper embedding model)
- Storing and searching for similar documents
- Displaying search results with similarity scores

## HTTP Client Usage

The `http_client.rs` example shows how to interact with a running Memista server using Rust's HTTP client:

```bash
# First, start the Memista server in another terminal:
cargo run

# Then run the HTTP client example:
cargo run --example http_client
```

This example demonstrates:
- Inserting chunks via the HTTP API
- Searching for similar chunks
- Dropping databases

## Running the Examples

To run any of these examples, you need to have the required build tools installed. On Ubuntu/Debian systems, you can install them with:

```bash
sudo apt install build-essential
```

Then you can run the examples as shown above.