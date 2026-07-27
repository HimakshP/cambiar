# Performance

## CSV → JSON baseline

Benchmarks are implemented using Criterion.

Current benchmark inputs cover tiny, small, medium, and large CSV files to measure both fixed conversion latency and sustained throughput.

Initial measurements show approximately 49 MiB/s throughput for ~1–10 MiB inputs on the development machine.

## Profiling

CPU profiling with `perf` and flamegraphs identified significant work in:

- CSV record parsing
- string allocation and copying
- JSON map construction
- JSON serialization
- filesystem/syscall activity

The current implementation constructs an intermediate `Vec<Value>` before serialization.

### Next experiment

Investigate streaming CSV records directly into JSON serialization to reduce intermediate allocations and peak memory usage. Re-run the same Criterion benchmarks to measure the effect.