# Memista Benchmarking Suite

This directory contains scripts for benchmarking Memista's performance.

## Prerequisites

1. Memista server running on `http://localhost:8083`
2. `wrk` HTTP benchmarking tool installed
3. `jq` JSON processor installed

## Benchmark Scripts

### insert_benchmark.sh

Measures the performance of inserting chunks into Memista.

Usage:
```bash
./insert_benchmark.sh [database_id] [num_chunks] [concurrency]
```

Default values:
- database_id: "benchmark_db"
- num_chunks: 1000
- concurrency: 10

### search_benchmark.sh

Measures the performance of searching for chunks in Memista.

Usage:
```bash
./search_benchmark.sh [database_id] [num_queries] [concurrency]
```

Default values:
- database_id: "benchmark_db"
- num_queries: 1000
- concurrency: 10

### concurrent_benchmark.sh

Measures the performance of concurrent insert and search operations.

Usage:
```bash
./concurrent_benchmark.sh [database_id] [num_operations] [concurrency]
```

Default values:
- database_id: "concurrent_benchmark_db"
- num_operations: 1000
- concurrency: 10

## Running Benchmarks

1. Start the Memista server:
   ```bash
   cargo run
   ```

2. In another terminal, run the benchmarks:
   ```bash
   cd benchmarks
   ./insert_benchmark.sh
   ./search_benchmark.sh
   ./concurrent_benchmark.sh
   ```

## Interpreting Results

Each benchmark script will output:
- Requests per second (RPS)
- Average latency
- 95th percentile latency
- 99th percentile latency

Higher RPS and lower latency values indicate better performance.