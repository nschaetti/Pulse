# Building Interfaces

Pulse interface construction is based on partition trees.

## Layout Tree

Use `LayoutNode` to define named zones and split them with constraints:

- `Constraint::Fixed(n)`
- `Constraint::Percent(p)`
- `Constraint::Fill`

Example shape:

- `root` (vertical)
  - `header` (`Fixed`)
  - `body` (`Fill`, horizontal)
    - `sidebar` (`Percent`)
    - `content` (`Fill`)
  - `footer` (`Fixed`)

Resolve with a frame-sized `Rect`, then fetch zones by name:

`resolved.area("content")`

## Padding

`Padding` shrinks a zone safely with saturation:

- `Padding::all(v)`
- `Padding::symmetric(vertical, horizontal)`

Use padding on layout nodes so render code stays clean.

## Text Primitive

`Text` is a lightweight renderer for single or multi-line content.

It is useful for:

- visualizing partition layouts,
- scaffolding interfaces before richer widgets,
- debugging area allocation.

## Block and List

`Block` draws a Unicode box (`┌ ┐ └ ┘ │ ─`) and optional title.

- Use `Block::new().title("...")` to frame sections.
- Use `block.inner_area(area)` to render content inside borders.

`List` renders selectable entries with basic scrolling.

- Use `List::new(items).selected(index)`.
- Selected entry is kept visible in the viewport.
- Rendering is clipped to the provided area.

## Reference Example

See `examples/interface_layout.rs` for a complete partitioned interface shell.
