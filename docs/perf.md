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
- `style_bench`: style-heavy redraw paths and widget-oriented rendering costs

## Style Bench Scenarios

- `style/full_repaint_120x40`: styled full-frame repaint baseline
- `style/style_only_diff_120x40`: same glyphs with style-only updates
- `style/mixed_widgets_frame`: `Paragraph`, `StatusBar`, and `Input` composition cost
- `style/input_edit_cycle`: edit helper throughput for cursor/value updates

## How to Compare Runs

Use a report-only workflow with median-focused comparisons:

1. Run `cargo bench` three times on the same machine profile.
2. Compare medians for the same benchmark names.
3. Treat small variance as noise; investigate sustained shifts.
4. Record notable changes in this file or release notes.

## Report-Only Policy

- Benchmarks do not fail CI right now.
- Results are used for trend tracking and regression visibility.
- When changing rendering or layout internals, run benchmarks locally and compare medians.
