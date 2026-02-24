use std::io;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use pulse::{
    run, App, Color, Command, Constraint, Direction, Frame, LayoutNode, List, ListStyle, Padding,
    Panel, PanelStyle, Rect, Slot, StatusBar, StatusBarStyle, Style, Text, Theme,
};

const NAV_ITEMS: [&str; 7] = [
    "Overview",
    "Metrics",
    "Alerts",
    "Deployments",
    "Logs",
    "Config",
    "About",
];

struct AdminConsole {
    layout: LayoutNode,
    selected: usize,
    theme_idx: usize,
    themes: [Theme; 3],
}

enum Msg {
    Up,
    Down,
    Theme(usize),
    Quit,
}

impl App for AdminConsole {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            Msg::Up => {
                self.selected = self.selected.saturating_sub(1);
                Command::none()
            }
            Msg::Down => {
                self.selected = (self.selected + 1).min(NAV_ITEMS.len().saturating_sub(1));
                Command::none()
            }
            Msg::Theme(next) => {
                self.theme_idx = next.min(self.themes.len().saturating_sub(1));
                Command::none()
            }
            Msg::Quit => Command::quit(),
        }
    }

    fn view(&self, frame: &mut Frame) {
        let root = Rect::new(0, 0, frame.width(), frame.height());
        let zones = self.layout.resolve(root);
        let theme = &self.themes[self.theme_idx];
        let panel_styles = PanelStyle::from_theme(theme);
        let list_styles = ListStyle::from_theme(theme);
        let status_styles = StatusBarStyle::from_theme(theme);

        if let Some(area) = zones.area("header") {
            Panel::new("Pulse Admin")
                .styles(panel_styles)
                .padding(Padding::all(1))
                .render(frame, area, |frame, inner| {
                    Text::new("Cluster: prod-eu-west | Status: healthy")
                        .style(theme.style_or("app.header.text", Style::new().fg(Color::Ansi(252))))
                        .render(frame, inner);
                });
        }

        if let Some(area) = zones.area("sidebar") {
            Panel::new("Navigation")
                .styles(panel_styles)
                .padding(Padding::all(1))
                .render(frame, area, |frame, inner| {
                    List::new(NAV_ITEMS)
                        .selected(self.selected)
                        .item_style(list_styles.item)
                        .selected_style(list_styles.selected)
                        .render(frame, inner);
                });
        }

        if let Some(area) = zones.area("content") {
            Panel::new("Panel")
                .styles(panel_styles)
                .padding(Padding::all(1))
                .render(frame, area, |frame, inner| {
                    Text::new(format!(
                        "Selected: {}\n\nUse this area to mount domain widgets.",
                        NAV_ITEMS[self.selected]
                    ))
                    .style(theme.style_or("text.primary", Style::new().fg(Color::Ansi(252))))
                    .render(frame, inner);
                });
        }

        if let Some(area) = zones.area("footer") {
            StatusBar::new()
                .left("up/down or j/k: navigate")
                .right("1/2/3: theme | q: quit")
                .style(status_styles.base)
                .left_style(status_styles.left)
                .right_style(status_styles.right)
                .margin(Padding::symmetric(0, 1))
                .render(frame, area);
        }
    }
}

fn build_layout() -> LayoutNode {
    LayoutNode::split(
        "root",
        Direction::Vertical,
        [
            Slot::new(
                Constraint::Fixed(3),
                LayoutNode::leaf("header").with_padding(Padding::symmetric(1, 2)),
            ),
            Slot::new(
                Constraint::Fill,
                LayoutNode::split(
                    "body",
                    Direction::Horizontal,
                    [
                        Slot::new(
                            Constraint::Percent(28),
                            LayoutNode::leaf("sidebar").with_padding(Padding::all(1)),
                        ),
                        Slot::new(
                            Constraint::Fill,
                            LayoutNode::leaf("content").with_padding(Padding::all(1)),
                        ),
                    ],
                ),
            ),
            Slot::new(
                Constraint::Fixed(1),
                LayoutNode::leaf("footer").with_padding(Padding::symmetric(0, 2)),
            ),
        ],
    )
}

fn map_key(key: KeyEvent) -> Option<Msg> {
    if key.kind != KeyEventKind::Press {
        return None;
    }

    match key.code {
        KeyCode::Up | KeyCode::Char('k') => Some(Msg::Up),
        KeyCode::Down | KeyCode::Char('j') => Some(Msg::Down),
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
    let mut app = AdminConsole {
        layout: build_layout(),
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
