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

> This converter is essentially just a wrapper around `htmd` crate. So no further optimization was done here.

## MD → TXT

| Dataset | Size | Time | Throughput |
|---------|-----:|-----:|-----------:|
| Tiny | ~95 B | 4.09 µs | 23.35 MiB/s |
| Small | ~10 KB | 14.48 µs | 712.97 MiB/s |
| Medium | ~1 MB | 2.38 ms | 419.76 MiB/s |
| Large | ~10 MB | 26.50 ms | 377.34 MiB/s |

> **Observation:** Markdown → TXT is currently the fastest converter in Cambiar, sustaining **~377 MiB/s** on large inputs. Profiling indicates that runtime is primarily dominated by `pulldown-cmark`'s Markdown parser rather than Cambiar's wrapper logic, leaving limited opportunity for architectural optimizations within Cambiar itself.


## PNG to JPG 

| Dataset |Size | Time | Throughput |
|---------|-----:|-----:|-----------:|
| Tiny | 1 kB |3.92 ms | 361.56 KiB/s |
| Small | 11 kB | 3.00 ms | 3.58 MiB/s |
| Medium | 980 kB| 99.85 ms | 9.55 MiB/s |
| Large | 10 MB | 403.54 ms | 24.85 MiB/s |

> **Observation:** The large benchmark showed a wider confidence interval than the smaller datasets, so its result has higher run-to-run variability.

## JPG to PNG

| Dataset |Size | Time | Throughput |
|---------|-----:|-----:|-----------:|
| Tiny | 1.42 kB | 38.987 µs | 24.462 MiB/s |
| Small | 10 kB | 87.604 µs | 108.86 MiB/s |
| Medium | 980 kB| 5.4730 ms | 174.25 MiB/s |
| Large | 10 MB | 1.4460 s | 6.7107 MiB/s |

> **Observation:** The throughput drop from medium to large file is due to the variation in the sample fixtures for each of the benches. Image converters rely on many factors for performance rather than just file sizes.

### Next experiment

Run Benchmarks and profiling for jpeg to png converter. Analyse flamegraph and form optimization hypotheses for the same.