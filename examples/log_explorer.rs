use std::io;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use pulse::{
    run, App, Block, Color, Command, Constraint, Direction, Frame, LayoutNode, List, Padding, Rect,
    Slot, Style, Text, Theme,
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

        if let Some(area) = zones.area("filters") {
            let block = panel_block(theme, "Filters");
            block.render(frame, area);
            Text::new("level=info  env=prod  since=15m")
                .style(style_from(
                    theme,
                    "text.muted",
                    Style::new().fg(Color::Ansi(251)),
                ))
                .render(frame, block.inner_area(area));
        }

        if let Some(area) = zones.area("sources") {
            let block = panel_block(theme, "Sources");
            block.render(frame, area);
            List::new(SOURCES)
                .selected(self.selected_source)
                .item_style(style_from(
                    theme,
                    "list.item",
                    Style::new().fg(Color::Ansi(252)),
                ))
                .selected_style(style_from(
                    theme,
                    "list.selected",
                    Style::new().fg(Color::Ansi(16)).bg(Color::Ansi(39)),
                ))
                .render(frame, block.inner_area(area));
        }

        if let Some(area) = zones.area("logs") {
            let block = panel_block(theme, "Logs");
            block.render(frame, area);

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
            Text::new(lines.join("\n"))
                .style(style_from(
                    theme,
                    "log.line",
                    Style::new().fg(Color::Ansi(252)),
                ))
                .render(frame, block.inner_area(area));
        }

        if let Some(area) = zones.area("status") {
            fill_area(
                frame,
                area,
                style_from(
                    theme,
                    "app.footer.bg",
                    Style::new().bg(Color::Rgb(28, 28, 28)),
                ),
            );
            Text::new("up/down or j/k: source | 1/2/3: theme | q: quit")
                .style(style_from(
                    theme,
                    "app.footer.text",
                    Style::new().fg(Color::Ansi(250)),
                ))
                .margin(Padding::symmetric(0, 1))
                .render(frame, area);
        }
    }
}

fn panel_block(theme: &Theme, title: &str) -> Block {
    Block::new()
        .title(title)
        .body_style(style_from(
            theme,
            "panel.body",
            Style::new().bg(Color::Rgb(22, 32, 56)),
        ))
        .border_style(style_from(
            theme,
            "panel.border",
            Style::new().fg(Color::Ansi(39)),
        ))
        .title_style(style_from(
            theme,
            "panel.title",
            Style::new().fg(Color::Rgb(200, 220, 255)),
        ))
        .padding(Padding::all(1))
}

fn style_from(theme: &Theme, token: &str, fallback: Style) -> Style {
    theme.style(token).unwrap_or(fallback)
}

fn fill_area(frame: &mut Frame, area: Rect, style: Style) {
    if area.width == 0 || area.height == 0 {
        return;
    }
    let line = " ".repeat(area.width as usize);
    frame.render_in(area, |f| {
        for y in 0..area.height {
            f.print_styled(0, y, &line, style);
        }
    });
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
