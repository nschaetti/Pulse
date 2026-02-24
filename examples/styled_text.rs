use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use pulse::{run, App, Color, Command, Frame, Modifier, Style};

struct StyledTextApp;

enum Msg {
    Quit,
}

impl App for StyledTextApp {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            Msg::Quit => Command::quit(),
        }
    }

    fn view(&self, frame: &mut Frame) {
        frame.print(0, 0, "Pulse Styled Text Demo");

        let ansi_style = Style::new().fg(Color::Ansi(39)).modifier(Modifier::Bold);
        frame.print_styled(0, 2, "ANSI 256 foreground (39) + bold", ansi_style);

        let rgb_style = Style::new()
            .fg(Color::Rgb(255, 230, 120))
            .bg(Color::Rgb(30, 50, 90))
            .modifier(Modifier::Underline);
        frame.print_styled(0, 4, "RGB fg/bg + underline", rgb_style);

        let reverse = Style::new()
            .fg(Color::Ansi(15))
            .bg(Color::Ansi(88))
            .modifier(Modifier::Reverse);
        frame.print_styled(0, 6, "Reverse style sample", reverse);

        frame.print(0, 8, "Press q to quit");
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
    let mut app = StyledTextApp;
    run(&mut app, map_key)
}
