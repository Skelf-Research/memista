# Interpreting Results

Understanding and analyzing Memista benchmark results.

## Key Metrics

### Requests Per Second (RPS)

Measures throughput - how many operations the server can handle.

```
Requests/sec: 15234.56
```

**Higher is better.** Indicates server capacity.

### Latency

Measures response time for individual requests.

| Metric | Description |
|--------|-------------|
| Average | Mean response time |
| p50 | Median (50th percentile) |
| p95 | 95% of requests faster than this |
| p99 | 99% of requests faster than this |

**Lower is better.** Indicates responsiveness.

### Example Output

```
Running Insert Benchmark...
  Requests/sec: 12456.78
  Avg Latency:  2.34ms
  p95 Latency:  5.67ms
  p99 Latency:  12.34ms
```

## Interpreting Insert Results

### Good Performance

```
Insert Benchmark:
  Requests/sec: 10000+
  Avg Latency:  < 5ms
  p99 Latency:  < 50ms
```

### Potential Issues

| Symptom | Possible Cause | Solution |
|---------|---------------|----------|
| Low RPS | Disk I/O bound | Use SSD, WAL mode |
| High p99 | Lock contention | Reduce concurrency |
| Degrading RPS | Index growth | Batch inserts |

## Interpreting Search Results

### Good Performance

```
Search Benchmark:
  Requests/sec: 50000+
  Avg Latency:  < 1ms
  p99 Latency:  < 10ms
```

### Factors Affecting Search

| Factor | Impact |
|--------|--------|
| Index size | Larger indexes = slower search |
| `num_results` | More results = more work |
| Embedding dimensions | Higher = slower |

## Interpreting Concurrent Results

Mixed workload performance:

```
Concurrent Benchmark:
  Total Ops/sec: 8000
  Insert Ops/sec: 3000
  Search Ops/sec: 5000
```

### Contention Indicators

- Large gap between insert and search RPS
- High variance in latency
- p99 >> p95 (long tail)

## Comparing Results

### Baseline Comparison

Run benchmarks on your target hardware and record baselines:

| Metric | Baseline | Current | Change |
|--------|----------|---------|--------|
| Insert RPS | 10000 | 12000 | +20% |
| Search RPS | 50000 | 48000 | -4% |
| p99 Latency | 15ms | 12ms | -20% |

### Configuration Comparison

Test different configurations:

```bash
# Test with WAL mode
DATABASE_PATH=test_wal.db ./benchmarks/run_all_benchmarks.sh

# Test with default mode
DATABASE_PATH=test_default.db ./benchmarks/run_all_benchmarks.sh
```

## Performance Tuning

### For Insert Performance

1. **Enable WAL mode** (default) - Better concurrent writes
2. **Batch inserts** - Reduce transaction overhead
3. **Use SSD storage** - Faster disk I/O

### For Search Performance

1. **Keep indexes in memory** - Faster than disk reads
2. **Limit `num_results`** - Return only what you need
3. **Use appropriate embedding dimensions** - Smaller = faster

### For Overall Performance

1. **Size database appropriately** - Don't over-provision
2. **Monitor memory usage** - Avoid swapping
3. **Use release builds** - 10-100x faster than debug

## Benchmarking Checklist

Before running benchmarks:

- [ ] Built with `--release`
- [ ] Server running in isolation
- [ ] Consistent test data
- [ ] Multiple runs for averaging
- [ ] Documented hardware specs

## Reporting Results

When sharing benchmark results, include:

```markdown
## Benchmark Results

**Hardware:**
- CPU: AMD Ryzen 9 5900X
- RAM: 32GB DDR4
- Storage: NVMe SSD

**Configuration:**
- Memista version: 0.1.0
- Database size: 100,000 chunks
- Embedding dimensions: 2

**Results:**
| Benchmark | RPS | Avg Latency | p99 Latency |
|-----------|-----|-------------|-------------|
| Insert | 12,456 | 2.3ms | 12ms |
| Search | 54,321 | 0.8ms | 5ms |
| Concurrent | 8,765 | 3.1ms | 18ms |
```
