use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use pulse::{run, App, Command, Frame};

struct ClippingApp;

enum Msg {
    Quit,
}

impl App for ClippingApp {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            Msg::Quit => Command::Quit,
        }
    }

    fn view(&self, frame: &mut Frame) {
        frame.print(0, 0, "Clipping Test");
        frame.print(0, 2, "Text near edges should be clipped safely.");

        let w = frame.width();
        let h = frame.height();

        if w > 0 {
            frame.print(w.saturating_sub(1), 4, "RIGHT EDGE");
        }
        if h > 0 {
            frame.print(0, h.saturating_sub(1), "BOTTOM EDGE");
        }

        frame.print(w.saturating_add(10), 1, "OUT OF BOUNDS X");
        frame.print(1, h.saturating_add(10), "OUT OF BOUNDS Y");
        frame.print(0, 6, "Press q to quit");
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
    let mut app = ClippingApp;
    run(&mut app, map_key)
}
