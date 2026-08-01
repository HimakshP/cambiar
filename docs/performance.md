# Performance

## Optimization Cycle

### First Cycle

Changes- 

• Change the convert() method to stream serialization directly, avoiding Vec declaration.

Results-

• Remove intermediate `Vec<Value>`

• Stream JSON objects directly to serializer

• Improve throughput by ~3.8% on 9.6 MB CSV

• Reduce intermediate allocations and memory footprint

### CSV → JSON

Benchmarks are implemented using Criterion.

Current benchmark inputs cover tiny, small, medium, and large CSV files to measure both fixed conversion latency and sustained throughput.


| Dataset | Size | Before (Time) | After (Time) | Before (Throughput) | After (Throughput) | Result |
|---------|-----:|--------------:|-------------:|--------------------:|-------------------:|:------|
| Tiny | 50 B | 26.36 µs | 26.67 µs | 1.81 MiB/s | 1.79 MiB/s |  ~2.5% slower |
| Small | 9.8 KB | 2.14 ms | 2.12 ms | 4.43 MiB/s | 4.48 MiB/s | ≈ No significant change |
| Medium | 978 KB | 19.48 ms | 19.07 ms | 49.03 MiB/s | 50.08 MiB/s | ~2.1% faster |
| Large | 9.6 MB | 195.31 ms | 188.07 ms | 48.83 MiB/s | 50.71 MiB/s |  ~3.8% faster |

## Profiling

CPU profiling with `perf` and flamegraphs identified significant work in:

- CSV record parsing
- string allocation and copying
- JSON map construction
- JSON serialization
- filesystem/syscall activity

The current implementation constructs an intermediate `Vec<Value>` before serialization.

### Next experiment

Run Benchmarks and profiling for html to markdown converter. Analyse flamegraph and form optimization hypotheses for the same.