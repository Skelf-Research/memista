# Benchmarking

This section covers performance testing and optimization for Memista.

## Overview

Memista includes a benchmarking suite to measure:

- **Insert performance** - Chunks inserted per second
- **Search performance** - Queries per second
- **Concurrent operations** - Performance under load

## Prerequisites

Install required tools:

```bash
# Ubuntu/Debian
sudo apt-get install wrk jq bc

# macOS
brew install wrk jq
```

## Quick Start

1. Start the Memista server:
   ```bash
   cargo run --release
   ```

2. Run all benchmarks:
   ```bash
   cd benchmarks
   ./run_all_benchmarks.sh
   ```

## Benchmark Scripts

| Script | Purpose |
|--------|---------|
| `insert_benchmark.sh` | Measures insertion throughput |
| `search_benchmark.sh` | Measures search latency and throughput |
| `concurrent_benchmark.sh` | Tests concurrent operations |
| `run_all_benchmarks.sh` | Runs all benchmarks sequentially |

## Metrics

Each benchmark reports:

- **Requests per second (RPS)** - Throughput measure
- **Average latency** - Mean response time
- **p95 latency** - 95th percentile response time
- **p99 latency** - 99th percentile response time

## Sections

- [Running Benchmarks](running-benchmarks.md) - Detailed instructions
- [Interpreting Results](interpreting-results.md) - Understanding the output
