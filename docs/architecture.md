# Pulse Architecture

Pulse follows an Elm-style update loop:

`Event -> Msg -> update -> Command -> view`

## Core Flow

1. Runtime emits an `Event` (`Key`, `Resize`, `Tick`).
2. Your mapper converts it into an app `Msg` (or ignores it).
3. `update(msg)` mutates state and returns a `Command`.
4. Runtime processes commands in deterministic FIFO order.
5. `view(frame)` renders a new frame snapshot.

## Commands

- `Command::none()` does nothing.
- `Command::emit(msg)` schedules one message.
- `Command::batch([...])` schedules multiple commands in order.
- `Command::quit()` exits the runtime loop.

Nested batches are supported and processed in stable order.

## Runtime Modes

- `run(...)`: compatibility API using key mapping.
- `run_with_events(...)`: preferred API for event-driven apps with configurable tick rate.

## Determinism Rules

- Message scheduling is FIFO.
- Batch execution is left-to-right.
- A `quit` command stops further processing immediately.

These rules keep update behavior predictable and easy to test.
