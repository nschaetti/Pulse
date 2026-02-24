use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use pulse::{run, App, Command, Frame};

struct KeymapApp {
    last_key: String,
    last_msg: &'static str,
}

enum Msg {
    Plus,
    Minus,
    Quit,
    Unknown(char),
}

impl App for KeymapApp {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            Msg::Plus => {
                self.last_msg = "Plus";
                Command::None
            }
            Msg::Minus => {
                self.last_msg = "Minus";
                Command::None
            }
            Msg::Quit => Command::Quit,
            Msg::Unknown(ch) => {
                self.last_msg = "Unknown";
                self.last_key.clear();
                self.last_key.push(ch);
                Command::None
            }
        }
    }

    fn view(&self, frame: &mut Frame) {
        frame.print(0, 0, "Keymap Test");
        frame.print(0, 2, "Mapped keys: '+' '-' 'q'");
        frame.print(0, 3, "Any other char is shown as Unknown");
        frame.print(0, 5, &format!("Last key: {}", self.last_key));
        frame.print(0, 6, &format!("Last msg: {}", self.last_msg));
        frame.print(0, 8, "Press q to quit");
    }
}

fn map_key(key: KeyEvent) -> Option<Msg> {
    if key.kind != KeyEventKind::Press {
        return None;
    }

    match key.code {
        KeyCode::Char('+') => Some(Msg::Plus),
        KeyCode::Char('-') => Some(Msg::Minus),
        KeyCode::Char('q') => Some(Msg::Quit),
        KeyCode::Char(ch) => Some(Msg::Unknown(ch)),
        _ => None,
    }
}

fn main() -> std::io::Result<()> {
    let mut app = KeymapApp {
        last_key: "<none>".to_string(),
        last_msg: "<none>",
    };

    run(&mut app, map_key)
}
