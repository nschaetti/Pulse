# Performance Baseline

Pulse uses Criterion benchmarks as a report-only baseline.

## Run Benchmarks

```bash
cargo bench
```

## Current Bench Suites

- `frame_bench`: redraw, partial updates, and nested clipping paths
- `layout_bench`: layout tree resolution with `Fixed`, `Percent`, and `Fill`
- `command_bench`: command scheduling and nested mapping behavior

## Report-Only Policy

- Benchmarks do not fail CI right now.
- Results are used for trend tracking and regression visibility.
- When changing rendering or layout internals, run benchmarks locally and compare medians.
