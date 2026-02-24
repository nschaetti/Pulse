use std::cell::Cell;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use pulse::{run, App, Command, Frame};

struct ResizeApp {
    draws: Cell<u64>,
}

enum Msg {
    Quit,
}

impl App for ResizeApp {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            Msg::Quit => Command::quit(),
        }
    }

    fn view(&self, frame: &mut Frame) {
        let next = self.draws.get().saturating_add(1);
        self.draws.set(next);

        frame.print(0, 0, "Resize Test");
        frame.print(0, 2, "Resize your terminal window.");
        frame.print(
            0,
            4,
            &format!("Current size: {} x {}", frame.width(), frame.height()),
        );
        frame.print(0, 5, &format!("Draw count: {}", self.draws.get()));
        frame.print(0, 7, "Press q to quit");
    }
}

fn map_key(key: KeyEvent) -> Option<Msg> {
    if key.kind != KeyEventKind::Press {
        return None;
    }

    match key.code {
        KeyCode::Char('q') => Some(Msg::Quit),
        _ => None,
    }
}

fn main() -> std::io::Result<()> {
    let mut app = ResizeApp {
        draws: Cell::new(0),
    };
    run(&mut app, map_key)
}
