use std::io;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use pulse::{
    run, App, Color, Command, Constraint, Direction, Frame, LayoutNode, List, ListStyle, Padding,
    Panel, PanelStyle, Paragraph, Rect, Slot, StatusBar, StatusBarStyle, Style, Text, Theme,
    WrapMode,
};

const SOURCES: [&str; 9] = [
    "api",
    "worker",
    "scheduler",
    "gateway",
    "auth",
    "billing",
    "notifications",
    "search",
    "storage",
];

struct LogExplorer {
    layout: LayoutNode,
    selected_source: usize,
    theme_idx: usize,
    themes: [Theme; 3],
}

enum Msg {
    Up,
    Down,
    Theme(usize),
    Quit,
}

impl App for LogExplorer {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            Msg::Up => {
                self.selected_source = self.selected_source.saturating_sub(1);
                Command::none()
            }
            Msg::Down => {
                self.selected_source =
                    (self.selected_source + 1).min(SOURCES.len().saturating_sub(1));
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

        if let Some(area) = zones.area("filters") {
            Panel::new("Filters")
                .styles(panel_styles)
                .padding(Padding::all(1))
                .render(frame, area, |frame, inner| {
                    Text::new("level=info  env=prod  since=15m")
                        .style(theme.style_or("text.muted", Style::new().fg(Color::Ansi(251))))
                        .render(frame, inner);
                });
        }

        if let Some(area) = zones.area("sources") {
            Panel::new("Sources")
                .styles(panel_styles)
                .padding(Padding::all(1))
                .render(frame, area, |frame, inner| {
                    List::new(SOURCES)
                        .selected(self.selected_source)
                        .item_style(list_styles.item)
                        .selected_style(list_styles.selected)
                        .render(frame, inner);
                });
        }

        if let Some(area) = zones.area("logs") {
            let source = SOURCES[self.selected_source];
            let lines = [
                format!("12:04:13 {} INFO  request completed in 14ms", source),
                format!("12:04:12 {} INFO  accepted connection", source),
                format!(
                    "12:04:11 {} WARN  retrying transient upstream error",
                    source
                ),
                format!("12:04:10 {} INFO  cache warmed", source),
                format!("12:04:09 {} INFO  background task heartbeat", source),
            ];

            Panel::new("Logs")
                .styles(panel_styles)
                .padding(Padding::all(1))
                .render(frame, area, |frame, inner| {
                    Paragraph::new(lines.join("\n"))
                        .wrap(WrapMode::NoWrap)
                        .style(theme.style_or(
                            "log.line",
                            theme.style_or("paragraph.default", Style::new().fg(Color::Ansi(252))),
                        ))
                        .render(frame, inner);
                });
        }

        if let Some(area) = zones.area("status") {
            StatusBar::new()
                .left("up/down or j/k: source")
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
                LayoutNode::leaf("filters").with_padding(Padding::symmetric(1, 2)),
            ),
            Slot::new(
                Constraint::Fill,
                LayoutNode::split(
                    "body",
                    Direction::Horizontal,
                    [
                        Slot::new(
                            Constraint::Percent(30),
                            LayoutNode::leaf("sources").with_padding(Padding::all(1)),
                        ),
                        Slot::new(
                            Constraint::Fill,
                            LayoutNode::leaf("logs").with_padding(Padding::all(1)),
                        ),
                    ],
                ),
            ),
            Slot::new(
                Constraint::Fixed(1),
                LayoutNode::leaf("status").with_padding(Padding::symmetric(0, 2)),
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
    let mut app = LogExplorer {
        layout: build_layout(),
        selected_source: 0,
        theme_idx: 0,
        themes: [
            load_theme("themes/default.json")?,
            load_theme("themes/warm.json")?,
            load_theme("themes/cool.json")?,
        ],
    };
    run(&mut app, map_key)
}
