use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use pulse::{run, App, Command, Frame, Rect};

struct LayoutDemo;

enum Msg {
    Quit,
}

impl App for LayoutDemo {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            Msg::Quit => Command::Quit,
        }
    }

    fn view(&self, frame: &mut Frame) {
        let root = Rect::new(0, 0, frame.width(), frame.height());
        let (top, bottom) = root.split_vertical(3);
        let (bottom_left, bottom_right) = bottom.split_horizontal(bottom.width / 2);

        frame.render_in(top, |frame| {
            frame.print(0, 0, "Top");
            frame.print(0, 1, &format!("{}x{}", top.width, top.height));
        });

        frame.render_in(bottom_left, |frame| {
            frame.print(0, 0, "Bottom Left");
            frame.print(
                0,
                1,
                &format!("{}x{}", bottom_left.width, bottom_left.height),
            );
        });

        frame.render_in(bottom_right, |frame| {
            frame.print(0, 0, "Bottom Right");
            frame.print(
                0,
                1,
                &format!("{}x{}", bottom_right.width, bottom_right.height),
            );
            frame.print(0, 3, "Press q to quit");
        });
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
    let mut app = LayoutDemo;
    run(&mut app, map_key)
}
