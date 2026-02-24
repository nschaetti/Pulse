# Migration to 0.2.0-alpha.1

This release introduces event-driven runtime and interface partition primitives while keeping temporary compatibility.

## Runtime

- Preferred: `run_with_events(app, tick_rate, map_event)`
- Compatibility: `run(app, map_key)` remains available

Use `run_with_events` for new apps to handle `Key`, `Resize`, and `Tick` consistently.

## Commands

Use helper constructors for clearer update code:

- `Command::none()`
- `Command::emit(msg)`
- `Command::batch([...])`
- `Command::quit()`

`Command::Batch` now supports ordered composition of multiple commands.

## Composition

For parent/child update routing, use `update_child`.

This avoids manual mapping for child `emit` commands and keeps parent message handling explicit.

## Interface Scaffolding

Start layout-heavy screens with:

- `LayoutNode`
- `Constraint::{Fixed, Percent, Fill}`
- `Padding`
- `Text`

Then replace `Text` blocks with higher-level widgets as they are introduced.

`0.2.0-alpha.1` also includes first baseline widgets:

- `Block` for titled bordered containers
- `List` for selectable, scrollable item views
