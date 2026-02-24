# Build an App End-to-End

This guide describes a practical Pulse flow for a themed terminal app.

## 1) Define layout zones

Use `LayoutNode` with `Fixed`, `Percent`, and `Fill` constraints to create named areas.

Typical structure:

- header
- sidebar
- content
- footer

## 2) Load a theme

Load JSON once at startup:

```rust
let theme = Theme::from_file("themes/default.json")?;
```

Use token lookups with fallback style values:

```rust
let text_style = theme.style_or("text.primary", Style::new());
```

## 3) Compose widgets

- `Panel` for bordered sections and inner area handling
- `List` for navigation/selection
- `Paragraph` for wrapped content
- `StatusBar` for one-line app status
- `Input` for editable query/filter fields

## 4) Manage focus and input

For editable screens (like settings):

- keep `input_focused: bool`
- route `InputEdit` only when focused
- keep list navigation active when input is not focused

## 5) Keep selection stable

When filtering list items:

- derive filtered items from current input value
- clamp selected index to filtered length
- render an empty-state message when no items match

## 6) Validate before release

Run quality checks:

```bash
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo check --examples
cargo bench --no-run
```

Reference implementation: `examples/settings.rs`.
