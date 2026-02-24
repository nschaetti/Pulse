# Styling

Pulse supports two styling paths:

- inline widget styles in Rust code
- external theme tokens loaded from JSON

## Inline styling

Use widget builders directly:

- `Text::style(...)`
- `Block::style(...)`, `border_style(...)`, `title_style(...)`, `body_style(...)`
- `List::item_style(...)`, `selected_style(...)`

Inline styles are useful for quick prototypes and local overrides.

## Theme JSON

Load a `Theme` from JSON and resolve token styles by name.

```rust
use pulse::Theme;

let theme = Theme::from_file("themes/default.json")?;
let list_selected = theme.style("list.selected");
```

### Token format

```json
{
  "tokens": {
    "list.item": { "fg": { "ansi": 252 } },
    "list.selected": {
      "fg": { "ansi": 16 },
      "bg": { "ansi": 39 },
      "modifiers": ["bold"]
    }
  }
}
```

### Supported color forms

- `{ "default": true }`
- `{ "ansi": 0..255 }`
- `{ "rgb": [r, g, b] }`

### Strict parsing

Theme parsing is strict:

- malformed colors fail
- unknown modifiers fail
- invalid token objects fail

This prevents silent style drift from bad configuration.

## Resolution policy

Recommended precedence in applications:

- widget defaults
- theme token style
- explicit inline override

Use this order to keep themes broad and local code intentional.

## Common tokens

Current examples share these tokens:

- `panel.body`, `panel.border`, `panel.title`
- `list.item`, `list.selected`
- `text.primary`, `text.muted`, `paragraph.default`, `paragraph.muted`, `log.line`
- `app.header.text`, `app.footer.bg`, `app.footer.text`
- `statusbar.bg`, `statusbar.left`, `statusbar.right`
- `input.bg`, `input.text`, `input.placeholder`, `input.cursor`, `input.focus`
- `settings.*` namespace for settings-specific panels

## Interactive pattern

For a themed settings screen with live filtering:

1. Keep an input value and cursor in app state.
2. Toggle input focus (for example with `/`).
3. Apply `InputEdit` only when input is focused.
4. Derive a filtered list from the current query.
5. Clamp the selected index when filtered results change.

See `examples/settings.rs` for a complete implementation using `Input`, `List`, and token-based styles.
