# Advanced Widgets

Pulse provides a second layer of widgets for richer terminal applications.

## Tabs

- Single-line tab navigation
- Active/inactive styles
- Margin and padding support

Use when an app has multiple top-level views.

## Table

- Header + rows rendering
- Row selection and scrolling
- Per-column alignment (`left`, `center`, `right`)
- Column widths with `Fixed` and `Fill`

Use for metrics, records, and admin views.

## FormField

- Wraps input controls with:
  - label
  - help text
  - error text

Use it with `Input` and future selection widgets.

## Theme tokens

Recommended tokens for advanced widgets:

- `tabs.bg`, `tabs.active`, `tabs.inactive`, `tabs.border`
- `table.header`, `table.row`, `table.selected`, `table.border`
- `field.label`, `field.help`, `field.error`

Examples:

- `examples/tabs_demo.rs`
- `examples/table_demo.rs`
- `examples/form_demo.rs`
