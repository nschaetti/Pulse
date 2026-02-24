# Component Composition

Pulse supports parent/child composition through explicit message routing.

## Pattern

1. Define child messages.
2. Wrap them in parent messages (for example `ParentMsg::Sidebar(...)`).
3. Delegate updates with `update_child`.

This keeps component boundaries clear while preserving a single app update loop.

## `update_child`

`update_child(child, msg, ParentMsg::Child)`:

- runs `child.update(msg)`,
- lifts any emitted child message to parent message type,
- preserves `none` and `quit` commands.

## Recommended Structure

- Parent owns layout orchestration and routing.
- Children own local state and rendering for their area.
- Parent `view` resolves zones, then calls child `view` per zone.

## Reference Example

See `examples/composition.rs` for a two-panel app with routed child messages.
