use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use pulse::{run, App, Command, Frame};

struct EmitApp {
    value: i32,
    updates: u32,
}

enum Msg {
    StartChain,
    AddOne,
    AddTen,
    Quit,
}

impl App for EmitApp {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        self.updates = self.updates.saturating_add(1);

        match msg {
            Msg::StartChain => Command::Emit(Msg::AddOne),
            Msg::AddOne => {
                self.value += 1;
                Command::Emit(Msg::AddTen)
            }
            Msg::AddTen => {
                self.value += 10;
                Command::None
            }
            Msg::Quit => Command::Quit,
        }
    }

    fn view(&self, frame: &mut Frame) {
        frame.print(0, 0, "Emit Chain Test");
        frame.print(0, 2, "Press 'e' to run: StartChain -> AddOne -> AddTen");
        frame.print(0, 4, &format!("Value: {}", self.value));
        frame.print(0, 5, &format!("Update calls: {}", self.updates));
        frame.print(0, 7, "Expected per 'e': Value +11 and 3 update calls");
        frame.print(0, 9, "Press q to quit");
    }
}

fn map_key(key: KeyEvent) -> Option<Msg> {
    if key.kind != KeyEventKind::Press {
        return None;
    }

    match key.code {
        KeyCode::Char('e') => Some(Msg::StartChain),
        KeyCode::Char('q') => Some(Msg::Quit),
        _ => None,
    }
}

fn main() -> std::io::Result<()> {
    let mut app = EmitApp {
        value: 0,
        updates: 0,
    };
    run(&mut app, map_key)
}
