use std::io;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use pulse::{
    apply_input_edit, run, App, Command, FormField, FormFieldStyle, Frame, Input, InputEdit,
    InputStyle, Padding, Panel, PanelStyle, StatusBar, StatusBarStyle, Text, Theme,
};

struct FormDemo {
    name: String,
    cursor: usize,
    focused: bool,
    theme_idx: usize,
    themes: [Theme; 3],
}

enum Msg {
    Edit(InputEdit),
    ToggleFocus,
    Theme(usize),
    Quit,
}

impl App for FormDemo {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            Msg::Edit(edit) => {
                if self.focused {
                    apply_input_edit(&mut self.name, &mut self.cursor, edit);
                }
                Command::none()
            }
            Msg::ToggleFocus => {
                self.focused = !self.focused;
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
        let panel = PanelStyle::from_theme(theme);
        let field = FormFieldStyle::from_theme(theme);
        let input = InputStyle::from_theme(theme);
        let status = StatusBarStyle::from_theme(theme);

        Panel::new("Form Demo")
            .styles(panel)
            .padding(Padding::all(1))
            .margin(Padding {
                top: 1,
                right: 2,
                bottom: 2,
                left: 2,
            })
            .render(
                frame,
                pulse::Rect::new(0, 0, frame.width(), frame.height().saturating_sub(1)),
                |frame, inner| {
                    FormField::new("Service name")
                        .style(field.base)
                        .label_style(field.label)
                        .help_style(field.help)
                        .error_style(field.error)
                        .help_text("Press / to focus/unfocus input")
                        .error_text(if self.name.trim().is_empty() {
                            "Name is required"
                        } else {
                            ""
                        })
                        .render(frame, inner, |frame, area| {
                            Input::new()
                                .value(self.name.clone())
                                .cursor(self.cursor)
                                .placeholder("example-service")
                                .focused(self.focused)
                                .style(input.base)
                                .focus_style(input.focus)
                                .placeholder_style(input.placeholder)
                                .cursor_style(input.cursor)
                                .render(frame, area);
                        });

                    Text::new(format!("Current value: {}", self.name))
                        .margin(Padding {
                            top: inner.height.saturating_sub(2),
                            right: 0,
                            bottom: 0,
                            left: 0,
                        })
                        .render(frame, inner);
                },
            );

        StatusBar::new()
            .left("/: focus input | type to edit")
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
        KeyCode::Left => Some(Msg::Edit(InputEdit::Left)),
        KeyCode::Right => Some(Msg::Edit(InputEdit::Right)),
        KeyCode::Backspace => Some(Msg::Edit(InputEdit::Backspace)),
        KeyCode::Home => Some(Msg::Edit(InputEdit::Home)),
        KeyCode::End => Some(Msg::Edit(InputEdit::End)),
        KeyCode::Char('/') => Some(Msg::ToggleFocus),
        KeyCode::Char('1') => Some(Msg::Theme(0)),
        KeyCode::Char('2') => Some(Msg::Theme(1)),
        KeyCode::Char('3') => Some(Msg::Theme(2)),
        KeyCode::Char('q') => Some(Msg::Quit),
        KeyCode::Char(ch) if !ch.is_control() => Some(Msg::Edit(InputEdit::Insert(ch))),
        _ => None,
    }
}

fn load_theme(path: &str) -> io::Result<Theme> {
    Theme::from_file(path).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
}

fn main() -> io::Result<()> {
    let mut app = FormDemo {
        name: String::new(),
        cursor: 0,
        focused: true,
        theme_idx: 0,
        themes: [
            load_theme("themes/default.json")?,
            load_theme("themes/warm.json")?,
            load_theme("themes/cool.json")?,
        ],
    };
    run(&mut app, map_key)
}
