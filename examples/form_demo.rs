use std::io;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use pulse::{
    apply_input_edit, run, App, Checkbox, CheckboxStyle, Command, FormField, FormFieldStyle, Frame,
    Input, InputEdit, InputStyle, MultiSelect, MultiSelectStyle, Padding, Panel, PanelStyle,
    ProgressBar, ProgressBarStyle, RadioGroup, RadioGroupStyle, Select, SelectStyle, Slider,
    SliderStyle, StatusBar, StatusBarStyle, Stepper, StepperStyle, Switch, SwitchStyle, Text,
    Theme,
};

struct FormDemo {
    name: String,
    cursor: usize,
    focus: Focus,
    env_selected: usize,
    env_highlight: usize,
    env_open: bool,
    auto_deploy: bool,
    strategy_selected: usize,
    strategy_highlight: usize,
    traffic_percent: u16,
    maintenance_mode: bool,
    retry_budget: u16,
    features_selected: Vec<usize>,
    features_highlight: usize,
    theme_idx: usize,
    themes: [Theme; 3],
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Focus {
    Name,
    Environment,
    AutoDeploy,
    Strategy,
    Traffic,
    Maintenance,
    Retry,
    Features,
}

const ENV_OPTIONS: [&str; 4] = ["dev", "stage", "preprod", "prod"];
const STRATEGY_OPTIONS: [&str; 3] = ["rolling", "canary", "blue-green"];
const FEATURE_OPTIONS: [&str; 4] = ["alerts", "backups", "audit", "tracing"];

enum Msg {
    Edit(InputEdit),
    NextFocus,
    SelectUp,
    SelectDown,
    SelectApply,
    ToggleCheckbox,
    Theme(usize),
    Quit,
}

impl App for FormDemo {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            Msg::Edit(edit) => {
                if self.focus == Focus::Name {
                    apply_input_edit(&mut self.name, &mut self.cursor, edit);
                } else if self.focus == Focus::Traffic {
                    match edit {
                        InputEdit::Left => {
                            self.traffic_percent = self.traffic_percent.saturating_sub(5)
                        }
                        InputEdit::Right => {
                            self.traffic_percent = (self.traffic_percent + 5).min(100)
                        }
                        InputEdit::Home => self.traffic_percent = 0,
                        InputEdit::End => self.traffic_percent = 100,
                        InputEdit::Insert(_) | InputEdit::Backspace => {}
                    }
                } else if self.focus == Focus::Retry {
                    match edit {
                        InputEdit::Left => self.retry_budget = self.retry_budget.saturating_sub(1),
                        InputEdit::Right => self.retry_budget = (self.retry_budget + 1).min(10),
                        InputEdit::Home => self.retry_budget = 0,
                        InputEdit::End => self.retry_budget = 10,
                        InputEdit::Insert(_) | InputEdit::Backspace => {}
                    }
                }
                Command::none()
            }
            Msg::NextFocus => {
                self.focus = if self.focus == Focus::Name {
                    Focus::Environment
                } else if self.focus == Focus::Environment {
                    Focus::AutoDeploy
                } else if self.focus == Focus::AutoDeploy {
                    Focus::Strategy
                } else if self.focus == Focus::Strategy {
                    Focus::Traffic
                } else if self.focus == Focus::Traffic {
                    Focus::Maintenance
                } else if self.focus == Focus::Maintenance {
                    Focus::Retry
                } else if self.focus == Focus::Retry {
                    Focus::Features
                } else {
                    Focus::Name
                };
                self.env_open = false;
                Command::none()
            }
            Msg::SelectUp => {
                if self.focus == Focus::Environment && self.env_open {
                    self.env_highlight = self.env_highlight.saturating_sub(1);
                } else if self.focus == Focus::Strategy {
                    self.strategy_highlight = self.strategy_highlight.saturating_sub(1);
                } else if self.focus == Focus::Features {
                    self.features_highlight = self.features_highlight.saturating_sub(1);
                }
                Command::none()
            }
            Msg::SelectDown => {
                if self.focus == Focus::Environment && self.env_open {
                    self.env_highlight =
                        (self.env_highlight + 1).min(ENV_OPTIONS.len().saturating_sub(1));
                } else if self.focus == Focus::Strategy {
                    self.strategy_highlight =
                        (self.strategy_highlight + 1).min(STRATEGY_OPTIONS.len().saturating_sub(1));
                } else if self.focus == Focus::Features {
                    self.features_highlight =
                        (self.features_highlight + 1).min(FEATURE_OPTIONS.len().saturating_sub(1));
                }
                Command::none()
            }
            Msg::SelectApply => {
                if self.focus == Focus::Environment {
                    if self.env_open {
                        self.env_selected = self.env_highlight.min(ENV_OPTIONS.len() - 1);
                        self.env_open = false;
                    } else {
                        self.env_open = true;
                        self.env_highlight = self.env_selected.min(ENV_OPTIONS.len() - 1);
                    }
                } else if self.focus == Focus::AutoDeploy {
                    self.auto_deploy = !self.auto_deploy;
                } else if self.focus == Focus::Strategy {
                    self.strategy_selected =
                        self.strategy_highlight.min(STRATEGY_OPTIONS.len() - 1);
                } else if self.focus == Focus::Maintenance {
                    self.maintenance_mode = !self.maintenance_mode;
                } else if self.focus == Focus::Features {
                    if self.features_selected.contains(&self.features_highlight) {
                        self.features_selected
                            .retain(|idx| *idx != self.features_highlight);
                    } else {
                        self.features_selected.push(self.features_highlight);
                        self.features_selected.sort_unstable();
                        self.features_selected.dedup();
                    }
                }
                Command::none()
            }
            Msg::ToggleCheckbox => {
                if self.focus == Focus::AutoDeploy {
                    self.auto_deploy = !self.auto_deploy;
                } else if self.focus == Focus::Maintenance {
                    self.maintenance_mode = !self.maintenance_mode;
                } else if self.focus == Focus::Features {
                    if self.features_selected.contains(&self.features_highlight) {
                        self.features_selected
                            .retain(|idx| *idx != self.features_highlight);
                    } else {
                        self.features_selected.push(self.features_highlight);
                        self.features_selected.sort_unstable();
                        self.features_selected.dedup();
                    }
                }
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
        let select = SelectStyle::from_theme(theme);
        let checkbox = CheckboxStyle::from_theme(theme);
        let radio = RadioGroupStyle::from_theme(theme);
        let slider = SliderStyle::from_theme(theme);
        let switch = SwitchStyle::from_theme(theme);
        let stepper = StepperStyle::from_theme(theme);
        let progress = ProgressBarStyle::from_theme(theme);
        let multiselect = MultiSelectStyle::from_theme(theme);
        let status = StatusBarStyle::from_theme(theme);

        let active_features = self
            .features_selected
            .iter()
            .filter_map(|idx| FEATURE_OPTIONS.get(*idx))
            .copied()
            .collect::<Vec<_>>()
            .join(",");

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
                    let (name_area, rest) = inner.split_vertical(4);
                    let (env_area, rest) = rest.split_vertical(if self.env_open { 6 } else { 4 });
                    let (toggle_area, rest) = rest.split_vertical(3);
                    let (strategy_area, value_area) = rest.split_vertical(5);
                    let (traffic_area, value_area) = value_area.split_vertical(3);
                    let (maintenance_area, value_area) = value_area.split_vertical(3);
                    let (retry_area, value_area) = value_area.split_vertical(3);
                    let (features_area, value_area) = value_area.split_vertical(5);
                    let (progress_area, value_area) = value_area.split_vertical(3);

                    let mut name_field = FormField::new("Service name")
                        .style(field.base)
                        .label_style(field.label)
                        .help_style(field.help)
                        .error_style(field.error)
                        .help_text("tab or /: switch field");
                    if self.name.trim().is_empty() {
                        name_field = name_field.error_text("Name is required");
                    }

                    name_field.render(frame, name_area, |frame, area| {
                        Input::new()
                            .value(self.name.clone())
                            .cursor(self.cursor)
                            .placeholder("example-service")
                            .focused(self.focus == Focus::Name)
                            .style(input.base)
                            .focus_style(input.focus)
                            .placeholder_style(input.placeholder)
                            .cursor_style(input.cursor)
                            .render(frame, area);
                    });

                    FormField::new("Environment")
                        .style(field.base)
                        .label_style(field.label)
                        .help_style(field.help)
                        .help_text("enter: open/apply, up/down: navigate")
                        .render(frame, env_area, |frame, area| {
                            Select::new(ENV_OPTIONS)
                                .selected(self.env_selected)
                                .highlighted(self.env_highlight)
                                .expanded(self.env_open)
                                .max_visible(4)
                                .placeholder("Select environment")
                                .style(select.base)
                                .selected_style(select.selected)
                                .dropdown_style(select.dropdown)
                                .highlight_style(select.highlight)
                                .render(frame, area);
                        });

                    FormField::new("Auto deploy")
                        .style(field.base)
                        .label_style(field.label)
                        .help_style(field.help)
                        .help_text("tab: focus | enter: toggle")
                        .render(frame, toggle_area, |frame, area| {
                            Checkbox::new("Enable rollout after deploy")
                                .checked(self.auto_deploy)
                                .focused(self.focus == Focus::AutoDeploy)
                                .style(checkbox.base)
                                .checked_style(checkbox.checked)
                                .box_style(checkbox.box_style)
                                .focus_style(checkbox.focus)
                                .render(frame, area);
                        });

                    FormField::new("Release strategy")
                        .style(field.base)
                        .label_style(field.label)
                        .help_style(field.help)
                        .help_text("up/down: highlight | enter: apply")
                        .render(frame, strategy_area, |frame, area| {
                            RadioGroup::new(STRATEGY_OPTIONS)
                                .selected(self.strategy_selected)
                                .highlighted(self.strategy_highlight)
                                .focused(self.focus == Focus::Strategy)
                                .max_visible(3)
                                .style(radio.base)
                                .selected_style(radio.selected)
                                .highlight_style(radio.highlight)
                                .marker_style(radio.marker)
                                .render(frame, area);
                        });

                    FormField::new("Traffic rollout")
                        .style(field.base)
                        .label_style(field.label)
                        .help_style(field.help)
                        .help_text("left/right: +/- 5 | home/end: min/max")
                        .render(frame, traffic_area, |frame, area| {
                            Slider::new(0, 100)
                                .value(self.traffic_percent)
                                .step(5)
                                .focused(self.focus == Focus::Traffic)
                                .style(slider.base)
                                .track_style(slider.track)
                                .fill_style(slider.fill)
                                .thumb_style(slider.thumb)
                                .focus_style(slider.focus)
                                .render(frame, area);
                        });

                    FormField::new("Maintenance mode")
                        .style(field.base)
                        .label_style(field.label)
                        .help_style(field.help)
                        .help_text("enter or space: toggle")
                        .render(frame, maintenance_area, |frame, area| {
                            Switch::new()
                                .on(self.maintenance_mode)
                                .focused(self.focus == Focus::Maintenance)
                                .style(switch.base)
                                .on_style(switch.on)
                                .off_style(switch.off)
                                .thumb_style(switch.thumb)
                                .focus_style(switch.focus)
                                .render(frame, area);
                        });

                    FormField::new("Retry budget")
                        .style(field.base)
                        .label_style(field.label)
                        .help_style(field.help)
                        .help_text("left/right: +/- 1 | home/end: min/max")
                        .render(frame, retry_area, |frame, area| {
                            Stepper::new(0, 10)
                                .value(self.retry_budget)
                                .step(1)
                                .focused(self.focus == Focus::Retry)
                                .style(stepper.base)
                                .value_style(stepper.value)
                                .controls_style(stepper.controls)
                                .focus_style(stepper.focus)
                                .render(frame, area);
                        });

                    FormField::new("Feature flags")
                        .style(field.base)
                        .label_style(field.label)
                        .help_style(field.help)
                        .help_text("up/down: highlight | enter/space: toggle")
                        .render(frame, features_area, |frame, area| {
                            MultiSelect::new(FEATURE_OPTIONS)
                                .selected(self.features_selected.clone())
                                .highlighted(self.features_highlight)
                                .focused(self.focus == Focus::Features)
                                .max_visible(4)
                                .style(multiselect.base)
                                .selected_style(multiselect.selected)
                                .highlight_style(multiselect.highlight)
                                .marker_style(multiselect.marker)
                                .render(frame, area);
                        });

                    FormField::new("Rollout progress")
                        .style(field.base)
                        .label_style(field.label)
                        .help_style(field.help)
                        .help_text("Derived from traffic rollout")
                        .render(frame, progress_area, |frame, area| {
                            ProgressBar::new()
                                .value(if self.maintenance_mode { 0 } else { self.traffic_percent })
                                .max(100)
                                .style(progress.base)
                                .track_style(progress.track)
                                .fill_style(progress.fill)
                                .label_style(progress.label)
                                .render(frame, area);
                        });

                    Text::new(format!(
                        "Current value: {} | env: {} | auto_deploy: {} | strategy: {} | traffic: {}% | maintenance: {} | retries: {} | features: {}",
                        self.name,
                        ENV_OPTIONS[self.env_selected],
                        if self.auto_deploy { "on" } else { "off" },
                        STRATEGY_OPTIONS[self.strategy_selected],
                        self.traffic_percent,
                        if self.maintenance_mode { "on" } else { "off" },
                        self.retry_budget,
                        active_features
                    ))
                    .margin(Padding {
                        top: value_area.y.saturating_sub(inner.y),
                        right: 0,
                        bottom: 0,
                        left: 0,
                    })
                    .render(frame, inner);
                },
            );

        StatusBar::new()
            .left(
                "tab or /: focus | up/down: list | left/right: traffic/retry | enter/space: toggle/apply",
            )
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
        KeyCode::Tab => Some(Msg::NextFocus),
        KeyCode::Up => Some(Msg::SelectUp),
        KeyCode::Down => Some(Msg::SelectDown),
        KeyCode::Enter => Some(Msg::SelectApply),
        KeyCode::Left => Some(Msg::Edit(InputEdit::Left)),
        KeyCode::Right => Some(Msg::Edit(InputEdit::Right)),
        KeyCode::Backspace => Some(Msg::Edit(InputEdit::Backspace)),
        KeyCode::Home => Some(Msg::Edit(InputEdit::Home)),
        KeyCode::End => Some(Msg::Edit(InputEdit::End)),
        KeyCode::Char('/') => Some(Msg::NextFocus),
        KeyCode::Char(' ') => Some(Msg::ToggleCheckbox),
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
        focus: Focus::Name,
        env_selected: 0,
        env_highlight: 0,
        env_open: false,
        auto_deploy: false,
        strategy_selected: 0,
        strategy_highlight: 0,
        traffic_percent: 35,
        maintenance_mode: false,
        retry_budget: 3,
        features_selected: vec![0, 2],
        features_highlight: 0,
        theme_idx: 0,
        themes: [
            load_theme("themes/default.json")?,
            load_theme("themes/warm.json")?,
            load_theme("themes/cool.json")?,
        ],
    };
    run(&mut app, map_key)
}
