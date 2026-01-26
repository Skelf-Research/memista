# Running Benchmarks

Detailed instructions for running Memista performance benchmarks.

## Setup

### 1. Build in Release Mode

Always benchmark with optimized builds:

```bash
cargo build --release
./target/release/memista
```

### 2. Install Dependencies

```bash
# Ubuntu/Debian
sudo apt-get install wrk jq bc

# macOS
brew install wrk jq

# Verify installation
wrk --version
jq --version
```

### 3. Start the Server

```bash
# Use release build for accurate results
./target/release/memista
```

## Insert Benchmark

Measures insertion performance.

### Usage

```bash
./benchmarks/insert_benchmark.sh [database_id] [num_chunks] [concurrency]
```

### Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `database_id` | `benchmark_db` | Target database |
| `num_chunks` | `1000` | Number of chunks to insert |
| `concurrency` | `10` | Concurrent connections |

### Example

```bash
# Default settings
./benchmarks/insert_benchmark.sh

# Custom settings
./benchmarks/insert_benchmark.sh my_db 5000 20
```

## Search Benchmark

Measures search performance.

### Usage

```bash
./benchmarks/search_benchmark.sh [database_id] [num_queries] [concurrency]
```

### Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `database_id` | `benchmark_db` | Database to search |
| `num_queries` | `1000` | Number of queries |
| `concurrency` | `10` | Concurrent connections |

### Example

```bash
# Ensure data exists first
./benchmarks/insert_benchmark.sh benchmark_db 1000 10

# Then run search benchmark
./benchmarks/search_benchmark.sh benchmark_db 1000 10
```

## Concurrent Benchmark

Tests mixed insert and search operations.

### Usage

```bash
./benchmarks/concurrent_benchmark.sh [database_id] [num_operations] [concurrency]
```

### Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `database_id` | `concurrent_benchmark_db` | Target database |
| `num_operations` | `1000` | Total operations |
| `concurrency` | `10` | Concurrent connections |

### Example

```bash
./benchmarks/concurrent_benchmark.sh test_db 2000 50
```

## Running All Benchmarks

The master script runs all benchmarks:

```bash
./benchmarks/run_all_benchmarks.sh
```

This executes:

1. Insert benchmark
2. Search benchmark (uses data from insert)
3. Concurrent benchmark
4. Cleanup

## Custom Benchmark Scripts

Create custom benchmarks using `wrk`:

```bash
#!/bin/bash

# Custom insert benchmark
wrk -t4 -c100 -d30s -s insert.lua http://localhost:8083/v1/insert
```

### Lua Script Example (insert.lua)

```lua
wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"
wrk.body = [[{
  "database_id": "benchmark_db",
  "chunks": [{
    "embedding": [0.1, 0.2],
    "text": "Benchmark text",
    "metadata": "{}"
  }]
}]]
```

## Environment Variables

Tune benchmark behavior:

```bash
# Set server for benchmarks
export MEMISTA_HOST=localhost
export MEMISTA_PORT=8083

# Run benchmarks
./benchmarks/run_all_benchmarks.sh
```

## Benchmark Best Practices

1. **Use release builds** - Debug builds are not representative
2. **Warm up** - Run a few queries before measuring
3. **Consistent environment** - Close other applications
4. **Multiple runs** - Average results across runs
5. **Vary parameters** - Test different concurrency levels

## Cleanup

After benchmarking, clean up test data:

```bash
curl -X DELETE http://localhost:8083/v1/drop \
  -H "Content-Type: application/json" \
  -d '{"database_id": "benchmark_db"}'
```
