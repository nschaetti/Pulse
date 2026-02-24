use std::io;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use pulse::{
    run, App, Color, Command, Constraint, Direction, Frame, LayoutNode, List, Padding, Rect, Slot,
    Style, Text, Theme,
};

const CATEGORIES: [&str; 8] = [
    "General",
    "Appearance",
    "Key Bindings",
    "Network",
    "Notifications",
    "Updates",
    "Advanced",
    "About",
];

#[derive(Clone, Copy)]
struct Palette {
    title_bg: Style,
    title_main: Style,
    title_sub: Style,
    categories_bg: Style,
    categories_title: Style,
    details_bg: Style,
    details_title: Style,
    details_section: Style,
    details_text: Style,
    footer_bg: Style,
    footer_text: Style,
    list_item: Style,
    list_selected: Style,
}

impl Palette {
    fn from_theme(theme: &Theme) -> Self {
        Self {
            title_bg: style_from(
                theme,
                "settings.title.bg",
                Style::new()
                    .fg(Color::Rgb(220, 230, 255))
                    .bg(Color::Rgb(34, 48, 86)),
            ),
            title_main: style_from(
                theme,
                "settings.title.main",
                Style::new().fg(Color::Rgb(255, 255, 255)),
            ),
            title_sub: style_from(
                theme,
                "settings.title.sub",
                Style::new().fg(Color::Ansi(153)),
            ),
            categories_bg: style_from(
                theme,
                "settings.categories.bg",
                Style::new().fg(Color::Ansi(252)).bg(Color::Rgb(22, 32, 56)),
            ),
            categories_title: style_from(
                theme,
                "settings.categories.title",
                Style::new().fg(Color::Rgb(200, 220, 255)),
            ),
            details_bg: style_from(
                theme,
                "settings.details.bg",
                Style::new().fg(Color::Ansi(195)).bg(Color::Rgb(12, 42, 46)),
            ),
            details_title: style_from(
                theme,
                "settings.details.title",
                Style::new().fg(Color::Rgb(180, 255, 240)),
            ),
            details_section: style_from(
                theme,
                "settings.details.section",
                Style::new().fg(Color::Rgb(240, 255, 240)),
            ),
            details_text: style_from(theme, "settings.details.text", Style::new()),
            footer_bg: style_from(
                theme,
                "settings.footer.bg",
                Style::new().fg(Color::Ansi(252)).bg(Color::Rgb(28, 28, 28)),
            ),
            footer_text: style_from(
                theme,
                "settings.footer.text",
                Style::new().fg(Color::Ansi(250)),
            ),
            list_item: style_from(
                theme,
                "list.item",
                Style::new().fg(Color::Ansi(252)).bg(Color::Rgb(22, 32, 56)),
            ),
            list_selected: style_from(
                theme,
                "list.selected",
                Style::new()
                    .fg(Color::Ansi(15))
                    .bg(Color::Rgb(72, 114, 186)),
            ),
        }
    }
}

fn style_from(theme: &Theme, token: &str, fallback: Style) -> Style {
    theme.style(token).unwrap_or(fallback)
}

struct SettingsApp {
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

impl App for SettingsApp {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            Msg::Up => {
                self.selected = self.selected.saturating_sub(1);
                Command::none()
            }
            Msg::Down => {
                self.selected = (self.selected + 1).min(CATEGORIES.len().saturating_sub(1));
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
        let palette = Palette::from_theme(&self.themes[self.theme_idx]);

        if let Some(area) = zones.area("title") {
            fill_area(frame, area, palette.title_bg);
            Text::new("Settings")
                .style(palette.title_main)
                .margin(Padding::symmetric(0, 1))
                .render(frame, area);
            Text::new("Pulse configuration center")
                .style(palette.title_sub)
                .margin(Padding {
                    top: 1,
                    right: 0,
                    bottom: 0,
                    left: 1,
                })
                .render(frame, area);
        }

        if let Some(area) = zones.area("categories") {
            fill_area(frame, area, palette.categories_bg);
            Text::new("Categories")
                .style(palette.categories_title)
                .margin(Padding::symmetric(0, 1))
                .render(frame, area);
            List::new(CATEGORIES)
                .selected(self.selected)
                .item_style(palette.list_item)
                .selected_style(palette.list_selected)
                .selected_prefix("›")
                .margin(Padding {
                    top: 2,
                    right: 1,
                    bottom: 0,
                    left: 1,
                })
                .render(frame, area);
        }

        if let Some(area) = zones.area("details") {
            fill_area(frame, area, palette.details_bg);
            Text::new("Details")
                .style(palette.details_title)
                .margin(Padding::symmetric(0, 1))
                .render(frame, area);
            Text::new(format!("Section: {}", CATEGORIES[self.selected]))
                .style(palette.details_section)
                .margin(Padding {
                    top: 2,
                    right: 1,
                    bottom: 0,
                    left: 1,
                })
                .render(frame, area);
            Text::new("- Placeholder option A\n- Placeholder option B\n- Placeholder option C")
                .style(palette.details_text)
                .margin(Padding {
                    top: 4,
                    right: 1,
                    bottom: 0,
                    left: 1,
                })
                .render(frame, area);
        }

        if let Some(area) = zones.area("footer") {
            fill_area(frame, area, palette.footer_bg);
            Text::new("up/down or j/k: select section | 1/2/3: theme | q: quit")
                .style(palette.footer_text)
                .margin(Padding::symmetric(0, 1))
                .render(frame, area);
        }
    }
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
                LayoutNode::leaf("title").with_padding(Padding::symmetric(1, 2)),
            ),
            Slot::new(
                Constraint::Fill,
                LayoutNode::split(
                    "body",
                    Direction::Horizontal,
                    [
                        Slot::new(
                            Constraint::Percent(35),
                            LayoutNode::leaf("categories").with_padding(Padding::all(1)),
                        ),
                        Slot::new(
                            Constraint::Fill,
                            LayoutNode::leaf("details").with_padding(Padding::all(1)),
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
    let mut app = SettingsApp {
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
