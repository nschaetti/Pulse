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
