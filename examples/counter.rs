use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use pulse::{run, App, Command, Frame};

struct Counter {
    value: i32,
}

enum Msg {
    Increment,
    Decrement,
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
            Msg::Decrement => {
                self.value -= 1;
                Command::none()
            }
            Msg::Quit => Command::quit(),
        }
    }

    fn view(&self, frame: &mut Frame) {
        frame.print(0, 0, "Pulse Counter");
        frame.print(0, 2, &format!("Value: {}", self.value));
        frame.print(0, 4, "Keys: '+' increment | '-' decrement | 'q' quit");
    }
}

fn map_key(key: KeyEvent) -> Option<Msg> {
    if key.kind != KeyEventKind::Press {
        return None;
    }

    match key.code {
        KeyCode::Char('+') => Some(Msg::Increment),
        KeyCode::Char('-') => Some(Msg::Decrement),
        KeyCode::Char('q') => Some(Msg::Quit),
        _ => None,
    }
}

fn main() -> std::io::Result<()> {
    let mut app = Counter { value: 0 };
    run(&mut app, map_key)
}
