use std::io;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use pulse::{
    run, Alignment, App, Command, Constraint, Frame, Padding, StatusBar, StatusBarStyle, Table,
    TableColumn, TableStyle, Theme,
};

struct TableDemo {
    selected: usize,
    theme_idx: usize,
    themes: [Theme; 3],
    rows: Vec<Vec<String>>,
}

enum Msg {
    Up,
    Down,
    Theme(usize),
    Quit,
}

impl App for TableDemo {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            Msg::Up => {
                self.selected = self.selected.saturating_sub(1);
                Command::none()
            }
            Msg::Down => {
                self.selected = (self.selected + 1).min(self.rows.len().saturating_sub(1));
                Command::none()
            }
            Msg::Theme(i) => {
                self.theme_idx = i.min(2);
                Command::none()
            }
            Msg::Quit => Command::quit(),
        }
    }

    fn view(&self, frame: &mut Frame) {
        let theme = &self.themes[self.theme_idx];
        let table_style = TableStyle::from_theme(theme);
        let status_style = StatusBarStyle::from_theme(theme);

        let columns = vec![
            TableColumn::new("Service", Constraint::Fixed(18)).align(Alignment::Left),
            TableColumn::new("Latency", Constraint::Fixed(10)).align(Alignment::Center),
            TableColumn::new("Error%", Constraint::Fixed(8)).align(Alignment::Right),
            TableColumn::new("Status", Constraint::Fill).align(Alignment::Left),
        ];

        Table::new(columns, self.rows.clone())
            .selected(self.selected)
            .style(table_style.base)
            .header_style(table_style.header)
            .row_style(table_style.row)
            .selected_style(table_style.selected)
            .border_style(table_style.border)
            .margin(Padding {
                top: 1,
                right: 2,
                bottom: 1,
                left: 2,
            })
            .render(
                frame,
                pulse::Rect::new(0, 0, frame.width(), frame.height().saturating_sub(1)),
            );

        StatusBar::new()
            .left("up/down: move selection")
            .right("1/2/3: theme | q: quit")
            .style(status_style.base)
            .left_style(status_style.left)
            .right_style(status_style.right)
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
        KeyCode::Up => Some(Msg::Up),
        KeyCode::Down => Some(Msg::Down),
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

fn sample_rows() -> Vec<Vec<String>> {
    (0..120)
        .map(|i| {
            vec![
                format!("service-{i:03}"),
                format!("{} ms", 10 + (i % 40)),
                format!("{:.2}", (i % 7) as f32 * 0.13),
                if i % 9 == 0 {
                    "degraded".to_string()
                } else {
                    "ok".to_string()
                },
            ]
        })
        .collect()
}

fn main() -> io::Result<()> {
    let mut app = TableDemo {
        selected: 0,
        theme_idx: 0,
        themes: [
            load_theme("themes/default.json")?,
            load_theme("themes/warm.json")?,
            load_theme("themes/cool.json")?,
        ],
        rows: sample_rows(),
    };

    run(&mut app, map_key)
}
