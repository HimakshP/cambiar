# Performance


## CSV → JSON

### First Cycle

Results-

• Remove intermediate `Vec<Value>`

• Stream JSON objects directly to serializer

• Improve throughput by ~3.8% on 9.6 MB CSV

• Reduce intermediate allocations and memory footprint


Benchmarks are implemented using Criterion.

Current benchmark inputs cover tiny, small, medium, and large CSV files to measure both fixed conversion latency and sustained throughput.


| Dataset | Size | Before (Time) | After (Time) | Before (Throughput) | After (Throughput) | Result |
|---------|-----:|--------------:|-------------:|--------------------:|-------------------:|:------|
| Tiny | 50 B | 26.36 µs | 26.67 µs | 1.81 MiB/s | 1.79 MiB/s |  ~2.5% slower |
| Small | 9.8 KB | 2.14 ms | 2.12 ms | 4.43 MiB/s | 4.48 MiB/s | ≈ No significant change |
| Medium | 978 KB | 19.48 ms | 19.07 ms | 49.03 MiB/s | 50.08 MiB/s | ~2.1% faster |
| Large | 9.6 MB | 195.31 ms | 188.07 ms | 48.83 MiB/s | 50.71 MiB/s |  ~3.8% faster |

### Profiling

CPU profiling with `perf` and flamegraphs identified significant work in:

- CSV record parsing
- string allocation and copying
- JSON map construction
- JSON serialization
- filesystem/syscall activity

Changed the convert() method to stream serialization directly, avoiding Vec declaration.
The previous implementation constructed an intermediate `Vec<Value>` before serialization.

## HTML -> MD

| Dataset | Size | Time | Throughput |
|---------|-----:|-----:|-----------:|
| Tiny | 99 B | 16.38 µs | 5.76 MiB/s |
| Small | 9.6 KB | 123.22 µs | 75.42 MiB/s |
| Medium | 821 KB | 25.76 ms | 31.11 MiB/s |
| Large | 8.1 MB | 274.94 ms | 29.21 MiB/s |

This converter is essentially just a wrapper around `htmd` crate. So no further optimization was done here.

### Next experiment

Run Benchmarks and profiling for markdown to text converter. Analyse flamegraph and form optimization hypotheses for the same.