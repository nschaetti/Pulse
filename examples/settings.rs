use std::io;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use pulse::{
    apply_input_edit, run, App, Color, Command, Constraint, Direction, Frame, Input, InputEdit,
    LayoutNode, List, Padding, Rect, Slot, Style, Text, Theme,
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
    input_bg: Style,
    input_text: Style,
    input_placeholder: Style,
    input_cursor: Style,
    input_focus: Style,
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
            input_bg: style_from(theme, "input.bg", Style::new().bg(Color::Rgb(18, 26, 48))),
            input_text: style_from(theme, "input.text", Style::new().fg(Color::Ansi(252))),
            input_placeholder: style_from(
                theme,
                "input.placeholder",
                Style::new().fg(Color::Ansi(244)),
            ),
            input_cursor: style_from(
                theme,
                "input.cursor",
                Style::new().bg(Color::Ansi(39)).fg(Color::Ansi(16)),
            ),
            input_focus: style_from(
                theme,
                "input.focus",
                Style::new().bg(Color::Rgb(28, 38, 68)),
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
    filter: String,
    cursor: usize,
    input_focused: bool,
    theme_idx: usize,
    themes: [Theme; 3],
}

enum Msg {
    Up,
    Down,
    Edit(InputEdit),
    ToggleInputFocus,
    Theme(usize),
    Quit,
}

impl App for SettingsApp {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            Msg::Up => {
                if !self.input_focused {
                    self.selected = self.selected.saturating_sub(1);
                    self.clamp_selected_to_filtered();
                }
                Command::none()
            }
            Msg::Down => {
                if !self.input_focused {
                    let len = self.filtered_categories().len();
                    if len > 0 {
                        self.selected = (self.selected + 1).min(len.saturating_sub(1));
                    }
                    self.clamp_selected_to_filtered();
                }
                Command::none()
            }
            Msg::Edit(edit) => {
                if self.input_focused {
                    apply_input_edit(&mut self.filter, &mut self.cursor, edit);
                    self.clamp_selected_to_filtered();
                }
                Command::none()
            }
            Msg::ToggleInputFocus => {
                self.input_focused = !self.input_focused;
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
        let filtered = self.filtered_categories();
        let selected_category = self.selected_category();

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
            if filtered.is_empty() {
                Text::new(format!("No results for '{}'", self.filter))
                    .style(palette.details_text)
                    .margin(Padding {
                        top: 2,
                        right: 1,
                        bottom: 0,
                        left: 1,
                    })
                    .render(frame, area);
            } else {
                List::new(filtered.iter().copied())
                    .selected(self.selected.min(filtered.len().saturating_sub(1)))
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
        }

        if let Some(area) = zones.area("details") {
            fill_area(frame, area, palette.details_bg);
            Text::new("Details")
                .style(palette.details_title)
                .margin(Padding::symmetric(0, 1))
                .render(frame, area);
            Text::new("Filter")
                .style(palette.details_section)
                .margin(Padding {
                    top: 2,
                    right: 1,
                    bottom: 0,
                    left: 1,
                })
                .render(frame, area);
            fill_area(
                frame,
                Rect::new(
                    area.x.saturating_add(1),
                    area.y.saturating_add(3),
                    area.width.saturating_sub(2),
                    1,
                ),
                palette.input_bg,
            );
            Input::new()
                .value(self.filter.clone())
                .cursor(self.cursor)
                .placeholder("type to filter categories")
                .focused(self.input_focused)
                .style(palette.input_text)
                .focus_style(palette.input_focus)
                .placeholder_style(palette.input_placeholder)
                .cursor_style(palette.input_cursor)
                .margin(Padding {
                    top: 3,
                    right: 1,
                    bottom: 0,
                    left: 1,
                })
                .render(frame, area);
            Text::new(match selected_category {
                Some(section) => format!(
                    "Section: {}\n\n- Placeholder option A\n- Placeholder option B\n- Placeholder option C",
                    section
                ),
                None => "No category selected\n\nAdjust the filter to see matching sections.".to_string(),
            })
                .style(palette.details_text)
                .margin(Padding {
                    top: 5,
                    right: 1,
                    bottom: 0,
                    left: 1,
                })
                .render(frame, area);
        }

        if let Some(area) = zones.area("footer") {
            fill_area(frame, area, palette.footer_bg);
            Text::new("up/down: section | /: focus filter | 1/2/3: theme | q: quit")
                .style(palette.footer_text)
                .margin(Padding::symmetric(0, 1))
                .render(frame, area);
        }
    }
}

impl SettingsApp {
    fn filtered_categories(&self) -> Vec<&'static str> {
        let query = self.filter.trim().to_lowercase();
        if query.is_empty() {
            return CATEGORIES.to_vec();
        }

        CATEGORIES
            .iter()
            .copied()
            .filter(|category| category.to_lowercase().contains(&query))
            .collect()
    }

    fn selected_category(&self) -> Option<&'static str> {
        let filtered = self.filtered_categories();
        if filtered.is_empty() {
            None
        } else {
            Some(filtered[self.selected.min(filtered.len().saturating_sub(1))])
        }
    }

    fn clamp_selected_to_filtered(&mut self) {
        let len = self.filtered_categories().len();
        if len == 0 {
            self.selected = 0;
            return;
        }
        self.selected = self.selected.min(len.saturating_sub(1));
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
        KeyCode::Left => Some(Msg::Edit(InputEdit::Left)),
        KeyCode::Right => Some(Msg::Edit(InputEdit::Right)),
        KeyCode::Backspace => Some(Msg::Edit(InputEdit::Backspace)),
        KeyCode::Home => Some(Msg::Edit(InputEdit::Home)),
        KeyCode::End => Some(Msg::Edit(InputEdit::End)),
        KeyCode::Char('/') => Some(Msg::ToggleInputFocus),
        KeyCode::Char('1') => Some(Msg::Theme(0)),
        KeyCode::Char('2') => Some(Msg::Theme(1)),
        KeyCode::Char('3') => Some(Msg::Theme(2)),
        KeyCode::Char('q') => Some(Msg::Quit),
        KeyCode::Char(ch) if ch != 'q' && !ch.is_control() => {
            Some(Msg::Edit(InputEdit::Insert(ch)))
        }
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
        filter: String::new(),
        cursor: 0,
        input_focused: false,
        theme_idx: 0,
        themes: [
            load_theme("themes/default.json")?,
            load_theme("themes/warm.json")?,
            load_theme("themes/cool.json")?,
        ],
    };
    run(&mut app, map_key)
}
