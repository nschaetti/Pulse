use std::io;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use pulse::{run, App, Command, Frame, Padding, StatusBar, StatusBarStyle, Tabs, TabsStyle, Theme};

const LABELS: [&str; 4] = ["Overview", "Metrics", "Logs", "Settings"];

struct TabsDemo {
    selected: usize,
    theme_idx: usize,
    themes: [Theme; 3],
}

enum Msg {
    Left,
    Right,
    Theme(usize),
    Quit,
}

impl App for TabsDemo {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            Msg::Left => {
                self.selected = self.selected.saturating_sub(1);
                Command::none()
            }
            Msg::Right => {
                self.selected = (self.selected + 1).min(LABELS.len().saturating_sub(1));
                Command::none()
            }
            Msg::Theme(index) => {
                self.theme_idx = index.min(2);
                Command::none()
            }
            Msg::Quit => Command::quit(),
        }
    }

    fn view(&self, frame: &mut Frame) {
        let theme = &self.themes[self.theme_idx];
        let tabs_style = TabsStyle::from_theme(theme);
        let status = StatusBarStyle::from_theme(theme);

        Tabs::new(LABELS)
            .selected(self.selected)
            .style(tabs_style.base)
            .active_style(tabs_style.active)
            .inactive_style(tabs_style.inactive)
            .border_style(tabs_style.border)
            .margin(Padding::symmetric(1, 2))
            .render(frame, pulse::Rect::new(0, 0, frame.width(), 1));

        frame.print(2, 3, &format!("Selected tab: {}", LABELS[self.selected]));

        StatusBar::new()
            .left("left/right: change tab")
            .right("1/2/3: theme | q: quit")
            .style(status.base)
            .left_style(status.left)
            .right_style(status.right)
            .margin(Padding::symmetric(0, 1))
            .render(
                frame,
                pulse::Rect::new(0, frame.height().saturating_sub(1), frame.width(), 1),
            );
    }
}

fn map_key(key: KeyEvent) -> Option<Msg> {
    if key.kind != KeyEventKind::Press {
        return None;
    }

    match key.code {
        KeyCode::Left => Some(Msg::Left),
        KeyCode::Right => Some(Msg::Right),
        KeyCode::Char('1') => Some(Msg::Theme(0)),
        KeyCode::Char('2') => Some(Msg::Theme(1)),
        KeyCode::Char('3') => Some(Msg::Theme(2)),
        KeyCode::Char('q') => Some(Msg::Quit),
        _ => None,
    }
}

fn load_theme(path: &str) -> io::Result<Theme> {
    Theme::from_file(path).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
}

fn main() -> io::Result<()> {
    let mut app = TabsDemo {
        selected: 0,
        theme_idx: 0,
        themes: [
            load_theme("themes/default.json")?,
            load_theme("themes/warm.json")?,
            load_theme("themes/cool.json")?,
        ],
    };
    run(&mut app, map_key)
}
