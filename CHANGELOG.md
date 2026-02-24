# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog, and this project follows Semantic Versioning.

## [0.2.0-alpha.1] - 2026-02-24

### Added

- Event-driven runtime API with `Event` (`Key`, `Resize`, `Tick`) and `run_with_events`.
- Temporary compatibility path via existing `run` key-mapping runtime.
- Command composition with `Command::Batch` and helper constructors (`none`, `emit`, `batch`, `quit`).
- Parent/child message lifting helper `update_child` for component composition.
- Interface partition tree API with `LayoutNode`, `Slot`, `Constraint` (`Fixed`, `Percent`, `Fill`), and `ResolvedLayout`.
- Layout primitives `Padding` and `Text` for interface scaffolding.
- Baseline widgets `Block` (Unicode box drawing with titles) and `List` (selectable with basic scrolling).
- Core style system with ANSI256/RGB colors, text modifiers, and styled frame rendering.
- Strict JSON theme loader with token-based style lookup (`Theme`).
- Additional widgets: `Paragraph`, `StatusBar`, and editable `Input` with cursor handling helpers.
- New examples: event runtime (`examples/events.rs`), component composition (`examples/composition.rs`), and interface shell (`examples/interface_layout.rs`).
- Criterion benchmark suites for frame, layout, and command paths (`benches/*.rs`).
- Report-only performance baseline documentation (`docs/perf.md`).
- Architecture and migration guides in `docs/`.

### Changed

- Runtime command processing now supports deterministic FIFO scheduling for `Emit` and nested `Batch` commands.
- README now documents the event-first flow and links to focused guides.
- `Text`, `Block`, and `List` now support inline style and spacing configuration for widget-level composition.
- `examples/settings.rs` now loads external theme files from `themes/*.json` and switches themes at runtime.
- `examples/admin_console.rs`, `examples/log_explorer.rs`, and `examples/interface_layout.rs` now use external JSON themes and runtime palette switching.
- `settings`, `log_explorer`, and footer bars now use the new widget layer (`Input`, `Paragraph`, `StatusBar`).

### Notes

- `0.2.0-alpha.1` targets architecture and developer ergonomics before richer widget primitives.
- `run` remains available for compatibility, but new applications should prefer `run_with_events`.

## [0.1.0-alpha.1] - 2026-02-24

### Added

- Initial Elm-inspired application architecture with `App` (`init`, `update`, `view`).
- Core runtime loop with terminal setup/teardown and key/resize event handling.
- `Command` flow with `None`, `Emit`, and `Quit`.
- `Frame` rendering buffer with clipping and scoped rendering (`render_in`).
- Layout primitive `Rect` with horizontal/vertical split helpers.
- Diff-based terminal backend that only redraws changed cells.
- Example applications for counter, key mapping, emit chains, clipping, layout, diffing, and resize behavior.
- Baseline unit tests for frame/layout/runtime behavior.
- Linux CI workflow (`check`, `fmt`, `clippy`, `test`).

### Notes

- `0.1.0-alpha.1` is an early pre-release focused on packaging and quality baseline.
- Public API remains intentionally small and may evolve before `1.0.0`.
