# Pulse

Pulse is a minimal Elm-inspired terminal UI library for Rust.

It focuses on a small architecture (`init -> update -> view`), deterministic state updates,
and efficient terminal rendering through frame diffing.

## Project status

- Current stage: `0.2.0-alpha.1`
- Focus: event-driven runtime, component composition, interface partitioning, perf baseline
- API is intentionally small and may evolve before a stable `1.0`

## Requirements

- Rust `1.74+`
- Linux terminal with ANSI support

## Quickstart

```rust
use std::time::Duration;

use crossterm::event::{KeyCode, KeyEventKind};
use pulse::{run_with_events, App, Command, Event, Frame};

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
                Command::none()
            }
            Msg::Quit => Command::quit(),
        }
    }

    fn view(&self, frame: &mut Frame) {
        frame.print(0, 0, "Pulse Counter");
        frame.print(0, 2, &format!("Value: {}", self.value));
        frame.print(0, 4, "Press '+' to increment, 'q' to quit");
    }
}

fn map_event(event: Event) -> Option<Msg> {
    match event {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('+') => Some(Msg::Increment),
            KeyCode::Char('q') => Some(Msg::Quit),
            _ => None,
        },
        _ => None,
    }
}

fn main() -> std::io::Result<()> {
    let mut app = Counter { value: 0 };
    run_with_events(&mut app, Duration::from_millis(250), map_event)
}
```

## Run examples

```bash
cargo run --example counter
cargo run --example keymap
cargo run --example emit
cargo run --example events
cargo run --example composition
cargo run --example interface_layout
cargo run --example styled_text
cargo run --example admin_console
cargo run --example settings
cargo run --example log_explorer
cargo run --example layout_demo
cargo run --example clipping
cargo run --example diff
cargo run --example resize
```

Theme-driven examples (`settings`, `admin_console`, `log_explorer`, `interface_layout`) switch palettes with `1/2/3`.

## Core API

- `App`: application contract with `init`, `update`, and `view`
- `Command`: post-update action (`none`, `emit`, `batch`, `quit` helpers)
- `Event`: runtime event type (`Key`, `Resize`, `Tick`)
- `Style`: text style with ANSI256/RGB colors and modifiers
- `Theme`: strict JSON token map for external styling
- `run_with_events`: preferred event-driven runtime with configurable tick rate
- `run`: compatibility runtime using a key mapper
- `Frame`: char buffer with clipping and scoped rendering (`render_in`)
- `Rect`: basic layout primitive with horizontal/vertical splits
- `LayoutNode` + `Constraint` (`Fixed`, `Percent`, `Fill`): partition trees for screen structure
- `Padding` + `Text`: simple primitives to visualize and populate resolved zones
- `Block` + `List`: baseline widgets for framed sections and scrollable selection
- `Component` and `update_child`: parent/child composition with lifted messages

Inline styling quick sample:

```rust
use pulse::{Block, Color, List, Padding, Style};

let panel = Block::new()
    .title("Navigation")
    .style(Style::new().bg(Color::Rgb(20, 28, 52)))
    .border_style(Style::new().fg(Color::Ansi(39)))
    .padding(Padding::all(1));

let list = List::new(["Overview", "Metrics", "Logs"])
    .selected(1)
    .item_style(Style::new().fg(Color::Ansi(252)))
    .selected_style(Style::new().fg(Color::Ansi(16)).bg(Color::Ansi(39)));
```

## Guides

- Architecture: `docs/architecture.md`
- Interface partitioning: `docs/interface.md`
- Parent/child composition: `docs/composition.md`
- Styling guide: `docs/styling.md`
- Migration notes: `docs/migration-0.2-alpha.md`
- Performance baseline: `docs/perf.md`

## Development checks

```bash
cargo check --all-targets
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo bench
```

Benchmark policy is report-only for now; see `docs/perf.md`.
