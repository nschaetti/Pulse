# Pulse

Pulse is a minimal Elm-inspired terminal UI library for Rust.

It focuses on a small architecture (`init -> update -> view`), deterministic state updates,
and efficient terminal rendering through frame diffing.

## Project status

- Current stage: `0.1.0-alpha.1`
- Phase 1 focus: packaging, baseline tests, Linux CI, and onboarding docs
- API is intentionally small and may evolve before a stable `1.0`

## Requirements

- Rust `1.74+`
- Linux terminal with ANSI support

## Quickstart

```rust
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use pulse::{run, App, Command, Frame};

struct Counter {
    value: i32,
}

enum Msg {
    Increment,
    Quit,
}

impl App for Counter {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            Msg::Increment => {
                self.value += 1;
                Command::None
            }
            Msg::Quit => Command::Quit,
        }
    }

    fn view(&self, frame: &mut Frame) {
        frame.print(0, 0, "Pulse Counter");
        frame.print(0, 2, &format!("Value: {}", self.value));
        frame.print(0, 4, "Press '+' to increment, 'q' to quit");
    }
}

fn map_key(key: KeyEvent) -> Option<Msg> {
    if key.kind != KeyEventKind::Press {
        return None;
    }

    match key.code {
        KeyCode::Char('+') => Some(Msg::Increment),
        KeyCode::Char('q') => Some(Msg::Quit),
        _ => None,
    }
}

fn main() -> std::io::Result<()> {
    let mut app = Counter { value: 0 };
    run(&mut app, map_key)
}
```

## Run examples

```bash
cargo run --example counter
cargo run --example keymap
cargo run --example emit
cargo run --example layout_demo
cargo run --example clipping
cargo run --example diff
cargo run --example resize
```

## Core API

- `App`: application contract with `init`, `update`, and `view`
- `Command`: post-update action (`None`, `Emit`, `Quit`)
- `run`: runtime loop handling key events, resize, and rendering
- `Frame`: char buffer with clipping and scoped rendering (`render_in`)
- `Rect`: basic layout primitive with horizontal/vertical splits

## Development checks

```bash
cargo check --all-targets
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```
