# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog, and this project follows Semantic Versioning.

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
