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

## Select

- Collapsed field + expandable dropdown list
- Separate selected and highlighted states for keyboard-driven UX
- Built-in viewport clamping for long option lists

Use for environment, profile, and mode selectors in forms.

## Checkbox

- Binary on/off form control
- Optional focused and checked style overrides
- Lightweight single-line rendering for dense forms

Use for deploy flags, feature toggles, and opt-in settings.

## RadioGroup

- Exclusive single-choice selector for multiple options
- Separate selected and highlighted rows for keyboard workflows
- Configurable viewport for compact forms

Use for release strategy, mode, and profile policy selection.

## Slider

- Numeric range control rendered on one line
- Fill + thumb visual feedback for current value
- Keyboard-friendly adjustments for form workflows

Use for percentages, thresholds, and rollout controls.

## Switch

- Compact binary control with explicit ON/OFF track states
- Focus and thumb styling for keyboard navigation clarity
- Form-friendly single-line rendering

Use for maintenance, safety mode, and operational toggles.

## Stepper

- Increment/decrement numeric control with explicit minus/plus affordances
- Bounded values with compact one-line rendering
- Works well for small integer settings in forms

Use for retry budgets, replica counts, and small numeric knobs.

## ProgressBar

- Read-only progress visualization with percentage label
- Track + fill styling for clear completion state
- Compact single-line rendering for form/status panels

Use for rollout, task completion, and sync progress indicators.

## MultiSelect

- Multi-choice selector with per-row toggle markers
- Keyboard highlight and selected states are independent
- Compact viewport behavior for long option sets

Use for feature flags, labels, and capability toggles.

## Theme tokens

Recommended tokens for advanced widgets:

- `tabs.bg`, `tabs.active`, `tabs.inactive`, `tabs.border`
- `table.header`, `table.row`, `table.selected`, `table.border`
- `field.label`, `field.help`, `field.error`
- `select.base`, `select.selected`, `select.dropdown`, `select.highlight`
- `checkbox.base`, `checkbox.checked`, `checkbox.box`, `checkbox.focus`
- `radio.base`, `radio.selected`, `radio.highlight`, `radio.marker`
- `slider.base`, `slider.track`, `slider.fill`, `slider.thumb`, `slider.focus`
- `switch.base`, `switch.on`, `switch.off`, `switch.thumb`, `switch.focus`
- `stepper.base`, `stepper.value`, `stepper.controls`, `stepper.focus`
- `progress.base`, `progress.track`, `progress.fill`, `progress.label`
- `multiselect.base`, `multiselect.selected`, `multiselect.highlight`, `multiselect.marker`

Examples:

- `examples/tabs_demo.rs`
- `examples/table_demo.rs`
- `examples/form_demo.rs`
