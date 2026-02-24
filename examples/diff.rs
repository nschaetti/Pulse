use std::cell::Cell;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use pulse::{run, App, Command, Frame};

struct DiffApp {
    x: i32,
    dx: i32,
    ticks: Cell<u64>,
}

enum Msg {
    Step,
    Quit,
}

impl App for DiffApp {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            Msg::Step => {
                self.x += self.dx;
                if self.x <= 0 {
                    self.x = 0;
                    self.dx = 1;
                } else if self.x >= 120 {
                    self.x = 120;
                    self.dx = -1;
                }
                Command::none()
            }
            Msg::Quit => Command::quit(),
        }
    }

    fn view(&self, frame: &mut Frame) {
        let ticks = self.ticks.get().saturating_add(1);
        self.ticks.set(ticks);

        frame.print(0, 0, "Diff Renderer Test");
        frame.print(0, 2, "Use space to move one step, 'q' quit.");
        frame.print(0, 4, &format!("Ticks: {}", self.ticks.get()));

        let max_x = frame.width().saturating_sub(1) as i32;
        let x = self.x.clamp(0, max_x) as u16;
        frame.print(x, 8, "@");
    }
}

fn map_key(key: KeyEvent) -> Option<Msg> {
    if key.kind != KeyEventKind::Press {
        return None;
    }

    match key.code {
        KeyCode::Char(' ') => Some(Msg::Step),
        KeyCode::Char('q') => Some(Msg::Quit),
        _ => None,
    }
}

fn main() -> std::io::Result<()> {
    let mut app = DiffApp {
        x: 0,
        dx: 1,
        ticks: Cell::new(0),
    };
    run(&mut app, map_key)
}
