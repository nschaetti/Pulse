use crate::{Color, Frame, Rect, Style, Theme};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Constraint {
    Fixed(u16),
    Percent(u8),
    Fill,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Padding {
    pub top: u16,
    pub right: u16,
    pub bottom: u16,
    pub left: u16,
}

impl Padding {
    pub fn all(value: u16) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    pub fn symmetric(vertical: u16, horizontal: u16) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }

    pub fn apply(&self, area: Rect) -> Rect {
        let left = self.left.min(area.width);
        let remaining_w = area.width.saturating_sub(left);
        let right = self.right.min(remaining_w);

        let top = self.top.min(area.height);
        let remaining_h = area.height.saturating_sub(top);
        let bottom = self.bottom.min(remaining_h);

        Rect::new(
            area.x.saturating_add(left),
            area.y.saturating_add(top),
            area.width.saturating_sub(left).saturating_sub(right),
            area.height.saturating_sub(top).saturating_sub(bottom),
        )
    }
}

#[derive(Clone, Debug)]
pub struct Slot {
    pub constraint: Constraint,
    pub node: LayoutNode,
}

impl Slot {
    pub fn new(constraint: Constraint, node: LayoutNode) -> Self {
        Self { constraint, node }
    }
}

#[derive(Clone, Debug)]
pub struct LayoutNode {
    name: String,
    padding: Padding,
    kind: NodeKind,
}

#[derive(Clone, Debug)]
enum NodeKind {
    Leaf,
    Split {
        direction: Direction,
        children: Vec<Slot>,
    },
}

impl LayoutNode {
    pub fn leaf(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            padding: Padding::default(),
            kind: NodeKind::Leaf,
        }
    }

    pub fn split(
        name: impl Into<String>,
        direction: Direction,
        children: impl IntoIterator<Item = Slot>,
    ) -> Self {
        Self {
            name: name.into(),
            padding: Padding::default(),
            kind: NodeKind::Split {
                direction,
                children: children.into_iter().collect(),
            },
        }
    }

    pub fn with_padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn resolve(&self, area: Rect) -> ResolvedLayout {
        let mut zones = Vec::new();
        resolve_node(self, area, &mut zones);
        ResolvedLayout { zones }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Zone {
    pub name: String,
    pub area: Rect,
}

#[derive(Clone, Debug, Default)]
pub struct ResolvedLayout {
    zones: Vec<Zone>,
}

impl ResolvedLayout {
    pub fn area(&self, name: &str) -> Option<Rect> {
        self.zones
            .iter()
            .rev()
            .find(|zone| zone.name == name)
            .map(|zone| zone.area)
    }

    pub fn zones(&self) -> &[Zone] {
        &self.zones
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Text {
    content: String,
    style: Style,
    padding: Padding,
    margin: Padding,
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            style: Style::default(),
            padding: Padding::default(),
            margin: Padding::default(),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn margin(mut self, margin: Padding) -> Self {
        self.margin = margin;
        self
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let area = self.padding.apply(self.margin.apply(area));
        frame.render_in(area, |frame| {
            for (y, line) in self.content.lines().enumerate() {
                if y as u16 >= area.height {
                    break;
                }
                frame.print_styled(0, y as u16, line, self.style);
            }
        });
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum WrapMode {
    #[default]
    Word,
    Char,
    NoWrap,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Alignment {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Paragraph {
    content: String,
    style: Style,
    padding: Padding,
    margin: Padding,
    wrap: WrapMode,
}

impl Paragraph {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            style: Style::default(),
            padding: Padding::default(),
            margin: Padding::default(),
            wrap: WrapMode::Word,
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn margin(mut self, margin: Padding) -> Self {
        self.margin = margin;
        self
    }

    pub fn wrap(mut self, wrap: WrapMode) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let area = self.padding.apply(self.margin.apply(area));
        if area.width == 0 || area.height == 0 {
            return;
        }

        let lines = wrap_lines(&self.content, area.width as usize, self.wrap);
        frame.render_in(area, |frame| {
            for (y, line) in lines.iter().enumerate() {
                if y as u16 >= area.height {
                    break;
                }
                frame.print_styled(0, y as u16, line, self.style);
            }
        });
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StatusBar {
    left: String,
    right: String,
    style: Style,
    left_style: Option<Style>,
    right_style: Option<Style>,
    margin: Padding,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            left: String::new(),
            right: String::new(),
            style: Style::default(),
            left_style: None,
            right_style: None,
            margin: Padding::default(),
        }
    }

    pub fn left(mut self, text: impl Into<String>) -> Self {
        self.left = text.into();
        self
    }

    pub fn right(mut self, text: impl Into<String>) -> Self {
        self.right = text.into();
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn left_style(mut self, style: Style) -> Self {
        self.left_style = Some(style);
        self
    }

    pub fn right_style(mut self, style: Style) -> Self {
        self.right_style = Some(style);
        self
    }

    pub fn margin(mut self, margin: Padding) -> Self {
        self.margin = margin;
        self
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let area = self.margin.apply(area);
        if area.width == 0 || area.height == 0 {
            return;
        }

        let width = area.width as usize;
        let base_style = self.style;
        let left_style = self.left_style.unwrap_or(base_style);
        let right_style = self.right_style.unwrap_or(base_style);

        let left = truncate_to_width(&self.left, width);
        let remaining = width.saturating_sub(left.chars().count());
        let right = truncate_to_width(&self.right, remaining);
        let right_width = right.chars().count();

        let mut row = " ".repeat(width);
        replace_segment(&mut row, 0, &left);
        if right_width > 0 && width >= right_width {
            replace_segment(&mut row, width - right_width, &right);
        }

        frame.render_in(area, |frame| {
            frame.print_styled(0, 0, &row, base_style);
            if !left.is_empty() {
                frame.print_styled(0, 0, &left, left_style);
            }
            if !right.is_empty() && width >= right_width {
                frame.print_styled((width - right_width) as u16, 0, &right, right_style);
            }
        });
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputEdit {
    Insert(char),
    Backspace,
    Left,
    Right,
    Home,
    End,
}

pub fn apply_input_edit(value: &mut String, cursor: &mut usize, edit: InputEdit) {
    let mut chars: Vec<char> = value.chars().collect();
    let mut cursor_pos = (*cursor).min(chars.len());

    match edit {
        InputEdit::Insert(ch) => {
            chars.insert(cursor_pos, ch);
            cursor_pos += 1;
        }
        InputEdit::Backspace => {
            if cursor_pos > 0 {
                chars.remove(cursor_pos - 1);
                cursor_pos -= 1;
            }
        }
        InputEdit::Left => {
            cursor_pos = cursor_pos.saturating_sub(1);
        }
        InputEdit::Right => {
            cursor_pos = (cursor_pos + 1).min(chars.len());
        }
        InputEdit::Home => {
            cursor_pos = 0;
        }
        InputEdit::End => {
            cursor_pos = chars.len();
        }
    }

    *value = chars.into_iter().collect();
    *cursor = cursor_pos;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Input {
    value: String,
    cursor: usize,
    placeholder: String,
    focused: bool,
    style: Style,
    placeholder_style: Option<Style>,
    cursor_style: Option<Style>,
    focus_style: Option<Style>,
    padding: Padding,
    margin: Padding,
}

impl Input {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            cursor: 0,
            placeholder: String::new(),
            focused: false,
            style: Style::default(),
            placeholder_style: None,
            cursor_style: None,
            focus_style: None,
            padding: Padding::default(),
            margin: Padding::default(),
        }
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self.cursor = self.cursor.min(self.value.chars().count());
        self
    }

    pub fn cursor(mut self, cursor: usize) -> Self {
        self.cursor = cursor;
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn placeholder_style(mut self, style: Style) -> Self {
        self.placeholder_style = Some(style);
        self
    }

    pub fn cursor_style(mut self, style: Style) -> Self {
        self.cursor_style = Some(style);
        self
    }

    pub fn focus_style(mut self, style: Style) -> Self {
        self.focus_style = Some(style);
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn margin(mut self, margin: Padding) -> Self {
        self.margin = margin;
        self
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let area = self.padding.apply(self.margin.apply(area));
        if area.width == 0 || area.height == 0 {
            return;
        }

        let width = area.width as usize;
        let base_style = if self.focused {
            self.focus_style.unwrap_or(self.style)
        } else {
            self.style
        };
        let cursor_style = self.cursor_style.unwrap_or(base_style);
        let placeholder_style = self.placeholder_style.unwrap_or(base_style);

        let mut row = " ".repeat(width);
        let (display, display_style) = if self.value.is_empty() {
            (&self.placeholder, placeholder_style)
        } else {
            (&self.value, base_style)
        };

        let clipped = truncate_to_width(display, width);
        replace_segment(&mut row, 0, &clipped);

        frame.render_in(area, |frame| {
            frame.print_styled(0, 0, &row, base_style);
            if !clipped.is_empty() {
                frame.print_styled(0, 0, &clipped, display_style);
            }

            if self.focused {
                let cursor_x = self.cursor.min(width.saturating_sub(1));
                let cursor_ch = if self.value.is_empty() {
                    clipped.chars().nth(cursor_x).unwrap_or(' ')
                } else {
                    self.value.chars().nth(cursor_x).unwrap_or(' ')
                };
                frame.print_styled(cursor_x as u16, 0, &cursor_ch.to_string(), cursor_style);
            }
        });
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PanelStyle {
    pub body: Style,
    pub border: Style,
    pub title: Style,
}

impl PanelStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        Self::from_theme_prefix(theme, "panel")
    }

    pub fn from_theme_prefix(theme: &Theme, prefix: &str) -> Self {
        Self {
            body: theme.style_or(
                &format!("{prefix}.body"),
                Style::new().bg(Color::Rgb(22, 32, 56)),
            ),
            border: theme.style_or(
                &format!("{prefix}.border"),
                Style::new().fg(Color::Ansi(39)),
            ),
            title: theme.style_or(
                &format!("{prefix}.title"),
                Style::new().fg(Color::Rgb(200, 220, 255)),
            ),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ListStyle {
    pub item: Style,
    pub selected: Style,
}

impl ListStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        Self {
            item: theme.style_or(
                "list.item",
                Style::new().fg(Color::Ansi(252)).bg(Color::Rgb(22, 32, 56)),
            ),
            selected: theme.style_or(
                "list.selected",
                Style::new().fg(Color::Ansi(16)).bg(Color::Ansi(39)),
            ),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SelectStyle {
    pub base: Style,
    pub selected: Style,
    pub dropdown: Style,
    pub highlight: Style,
}

impl SelectStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        Self {
            base: theme.style_or(
                "select.base",
                Style::new().fg(Color::Ansi(252)).bg(Color::Rgb(18, 26, 48)),
            ),
            selected: theme.style_or(
                "select.selected",
                Style::new().fg(Color::Ansi(16)).bg(Color::Ansi(39)),
            ),
            dropdown: theme.style_or(
                "select.dropdown",
                Style::new().fg(Color::Ansi(252)).bg(Color::Rgb(22, 32, 56)),
            ),
            highlight: theme.style_or(
                "select.highlight",
                Style::new().fg(Color::Ansi(16)).bg(Color::Ansi(39)),
            ),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckboxStyle {
    pub base: Style,
    pub checked: Style,
    pub box_style: Style,
    pub focus: Style,
}

impl CheckboxStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        Self {
            base: theme.style_or("checkbox.base", Style::new().fg(Color::Ansi(252))),
            checked: theme.style_or(
                "checkbox.checked",
                Style::new().fg(Color::Ansi(16)).bg(Color::Ansi(39)),
            ),
            box_style: theme.style_or("checkbox.box", Style::new().fg(Color::Ansi(39))),
            focus: theme.style_or(
                "checkbox.focus",
                Style::new().fg(Color::Ansi(16)).bg(Color::Ansi(45)),
            ),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RadioGroupStyle {
    pub base: Style,
    pub selected: Style,
    pub highlight: Style,
    pub marker: Style,
}

impl RadioGroupStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        Self {
            base: theme.style_or("radio.base", Style::new().fg(Color::Ansi(252))),
            selected: theme.style_or(
                "radio.selected",
                Style::new().fg(Color::Ansi(16)).bg(Color::Ansi(39)),
            ),
            highlight: theme.style_or(
                "radio.highlight",
                Style::new().fg(Color::Ansi(16)).bg(Color::Ansi(45)),
            ),
            marker: theme.style_or("radio.marker", Style::new().fg(Color::Ansi(39))),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SliderStyle {
    pub base: Style,
    pub track: Style,
    pub fill: Style,
    pub thumb: Style,
    pub focus: Style,
}

impl SliderStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        Self {
            base: theme.style_or("slider.base", Style::new().fg(Color::Ansi(252))),
            track: theme.style_or("slider.track", Style::new().fg(Color::Ansi(244))),
            fill: theme.style_or("slider.fill", Style::new().fg(Color::Ansi(39))),
            thumb: theme.style_or("slider.thumb", Style::new().fg(Color::Ansi(39))),
            focus: theme.style_or(
                "slider.focus",
                Style::new().fg(Color::Ansi(16)).bg(Color::Ansi(45)),
            ),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SwitchStyle {
    pub base: Style,
    pub on: Style,
    pub off: Style,
    pub thumb: Style,
    pub focus: Style,
}

impl SwitchStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        Self {
            base: theme.style_or("switch.base", Style::new().fg(Color::Ansi(252))),
            on: theme.style_or(
                "switch.on",
                Style::new().fg(Color::Ansi(16)).bg(Color::Ansi(39)),
            ),
            off: theme.style_or(
                "switch.off",
                Style::new().fg(Color::Ansi(252)).bg(Color::Ansi(238)),
            ),
            thumb: theme.style_or("switch.thumb", Style::new().fg(Color::Ansi(231))),
            focus: theme.style_or(
                "switch.focus",
                Style::new().fg(Color::Ansi(16)).bg(Color::Ansi(45)),
            ),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StepperStyle {
    pub base: Style,
    pub value: Style,
    pub controls: Style,
    pub focus: Style,
}

impl StepperStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        Self {
            base: theme.style_or("stepper.base", Style::new().fg(Color::Ansi(252))),
            value: theme.style_or("stepper.value", Style::new().fg(Color::Ansi(252))),
            controls: theme.style_or("stepper.controls", Style::new().fg(Color::Ansi(39))),
            focus: theme.style_or(
                "stepper.focus",
                Style::new().fg(Color::Ansi(16)).bg(Color::Ansi(45)),
            ),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProgressBarStyle {
    pub base: Style,
    pub track: Style,
    pub fill: Style,
    pub label: Style,
}

impl ProgressBarStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        Self {
            base: theme.style_or("progress.base", Style::new().fg(Color::Ansi(252))),
            track: theme.style_or("progress.track", Style::new().fg(Color::Ansi(244))),
            fill: theme.style_or("progress.fill", Style::new().fg(Color::Ansi(39))),
            label: theme.style_or("progress.label", Style::new().fg(Color::Ansi(252))),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MultiSelectStyle {
    pub base: Style,
    pub selected: Style,
    pub highlight: Style,
    pub marker: Style,
}

impl MultiSelectStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        Self {
            base: theme.style_or("multiselect.base", Style::new().fg(Color::Ansi(252))),
            selected: theme.style_or(
                "multiselect.selected",
                Style::new().fg(Color::Ansi(16)).bg(Color::Ansi(39)),
            ),
            highlight: theme.style_or(
                "multiselect.highlight",
                Style::new().fg(Color::Ansi(16)).bg(Color::Ansi(45)),
            ),
            marker: theme.style_or("multiselect.marker", Style::new().fg(Color::Ansi(39))),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StatusBarStyle {
    pub base: Style,
    pub left: Style,
    pub right: Style,
}

impl StatusBarStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        let fallback_base =
            theme.style_or("app.footer.bg", Style::new().bg(Color::Rgb(28, 28, 28)));
        let fallback_text = theme.style_or("app.footer.text", Style::new().fg(Color::Ansi(250)));

        Self {
            base: theme.style_or("statusbar.bg", fallback_base),
            left: theme.style_or("statusbar.left", fallback_text),
            right: theme.style_or("statusbar.right", fallback_text),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InputStyle {
    pub base: Style,
    pub focus: Style,
    pub placeholder: Style,
    pub cursor: Style,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TabsStyle {
    pub base: Style,
    pub active: Style,
    pub inactive: Style,
    pub border: Style,
}

impl TabsStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        Self {
            base: theme.style_or("tabs.bg", Style::new().bg(Color::Rgb(20, 24, 44))),
            active: theme.style_or(
                "tabs.active",
                Style::new().fg(Color::Ansi(16)).bg(Color::Ansi(39)),
            ),
            inactive: theme.style_or("tabs.inactive", Style::new().fg(Color::Ansi(252))),
            border: theme.style_or("tabs.border", Style::new().fg(Color::Ansi(39))),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TableStyle {
    pub base: Style,
    pub header: Style,
    pub row: Style,
    pub selected: Style,
    pub border: Style,
}

impl TableStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        Self {
            base: Style::default(),
            header: theme.style_or(
                "table.header",
                Style::new().fg(Color::Ansi(230)).bg(Color::Rgb(26, 34, 58)),
            ),
            row: theme.style_or("table.row", Style::new().fg(Color::Ansi(252))),
            selected: theme.style_or(
                "table.selected",
                Style::new().fg(Color::Ansi(16)).bg(Color::Ansi(39)),
            ),
            border: theme.style_or("table.border", Style::new().fg(Color::Ansi(39))),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FormFieldStyle {
    pub base: Style,
    pub label: Style,
    pub help: Style,
    pub error: Style,
}

impl FormFieldStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        Self {
            base: Style::default(),
            label: theme.style_or("field.label", Style::new().fg(Color::Ansi(252))),
            help: theme.style_or("field.help", Style::new().fg(Color::Ansi(244))),
            error: theme.style_or("field.error", Style::new().fg(Color::Ansi(196))),
        }
    }
}

impl InputStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        Self {
            base: theme.style_or("input.text", Style::new().fg(Color::Ansi(252))),
            focus: theme.style_or("input.focus", Style::new().bg(Color::Rgb(28, 38, 68))),
            placeholder: theme.style_or("input.placeholder", Style::new().fg(Color::Ansi(244))),
            cursor: theme.style_or(
                "input.cursor",
                Style::new().fg(Color::Ansi(16)).bg(Color::Ansi(39)),
            ),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Panel {
    block: Block,
}

impl Panel {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            block: Block::new().title(title),
        }
    }

    pub fn block(mut self, block: Block) -> Self {
        self.block = block;
        self
    }

    pub fn styles(mut self, styles: PanelStyle) -> Self {
        self.block = self
            .block
            .body_style(styles.body)
            .border_style(styles.border)
            .title_style(styles.title);
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.block = self.block.padding(padding);
        self
    }

    pub fn margin(mut self, margin: Padding) -> Self {
        self.block = self.block.margin(margin);
        self
    }

    pub fn render(
        &self,
        frame: &mut Frame,
        area: Rect,
        render_inner: impl FnOnce(&mut Frame, Rect),
    ) {
        self.block.render(frame, area);
        render_inner(frame, self.block.inner_area(area));
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum BorderType {
    #[default]
    Unicode,
    Ascii,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Borders {
    pub top: bool,
    pub right: bool,
    pub bottom: bool,
    pub left: bool,
}

impl Default for Borders {
    fn default() -> Self {
        Self::all()
    }
}

impl Borders {
    pub fn all() -> Self {
        Self {
            top: true,
            right: true,
            bottom: true,
            left: true,
        }
    }

    pub fn none() -> Self {
        Self {
            top: false,
            right: false,
            bottom: false,
            left: false,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Block {
    title: Option<String>,
    style: Style,
    border_style: Option<Style>,
    title_style: Option<Style>,
    body_style: Option<Style>,
    padding: Padding,
    margin: Padding,
    border_type: BorderType,
    borders: Borders,
}

impl Block {
    pub fn new() -> Self {
        Self {
            title: None,
            style: Style::default(),
            border_style: None,
            title_style: None,
            body_style: None,
            padding: Padding::default(),
            margin: Padding::default(),
            border_type: BorderType::Unicode,
            borders: Borders::all(),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn border_style(mut self, style: Style) -> Self {
        self.border_style = Some(style);
        self
    }

    pub fn title_style(mut self, style: Style) -> Self {
        self.title_style = Some(style);
        self
    }

    pub fn body_style(mut self, style: Style) -> Self {
        self.body_style = Some(style);
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn margin(mut self, margin: Padding) -> Self {
        self.margin = margin;
        self
    }

    pub fn border_type(mut self, border_type: BorderType) -> Self {
        self.border_type = border_type;
        self
    }

    pub fn borders(mut self, borders: Borders) -> Self {
        self.borders = borders;
        self
    }

    pub fn inner_area(&self, area: Rect) -> Rect {
        let mut area = self.margin.apply(area);

        let left = u16::from(self.borders.left);
        let right = u16::from(self.borders.right);
        let top = u16::from(self.borders.top);
        let bottom = u16::from(self.borders.bottom);

        area = Rect::new(
            area.x.saturating_add(left),
            area.y.saturating_add(top),
            area.width.saturating_sub(left).saturating_sub(right),
            area.height.saturating_sub(top).saturating_sub(bottom),
        );

        self.padding.apply(area)
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let area = self.margin.apply(area);
        if area.width == 0 || area.height == 0 {
            return;
        }

        let body_style = self.body_style.unwrap_or(self.style);
        fill_with_style(frame, area, body_style);

        let border_style = self.border_style.unwrap_or(self.style);
        let title_style = self.title_style.unwrap_or(border_style);
        let glyphs = border_glyphs(self.border_type);

        frame.render_in(area, |frame| {
            let left_x = 0;
            let right_x = area.width.saturating_sub(1);
            let top_y = 0;
            let bottom_y = area.height.saturating_sub(1);

            if self.borders.top {
                frame.print_styled(left_x, top_y, glyphs.top_left, border_style);
                if area.width > 2 {
                    frame.print_styled(
                        1,
                        top_y,
                        &glyphs
                            .horizontal
                            .repeat(area.width.saturating_sub(2) as usize),
                        border_style,
                    );
                }
                frame.print_styled(right_x, top_y, glyphs.top_right, border_style);
            }

            if self.borders.bottom {
                frame.print_styled(left_x, bottom_y, glyphs.bottom_left, border_style);
                if area.width > 2 {
                    frame.print_styled(
                        1,
                        bottom_y,
                        &glyphs
                            .horizontal
                            .repeat(area.width.saturating_sub(2) as usize),
                        border_style,
                    );
                }
                frame.print_styled(right_x, bottom_y, glyphs.bottom_right, border_style);
            }

            if self.borders.left {
                for y in 1..area.height.saturating_sub(1) {
                    frame.print_styled(left_x, y, glyphs.vertical, border_style);
                }
            }

            if self.borders.right {
                for y in 1..area.height.saturating_sub(1) {
                    frame.print_styled(right_x, y, glyphs.vertical, border_style);
                }
            }

            if let Some(title) = &self.title {
                let available =
                    area.width
                        .saturating_sub(u16::from(self.borders.left))
                        .saturating_sub(u16::from(self.borders.right)) as usize;
                if available > 0 {
                    let decorated = format!(" {} ", title);
                    let truncated: String = decorated.chars().take(available).collect();
                    let title_x = u16::from(self.borders.left);
                    frame.print_styled(title_x, 0, &truncated, title_style);
                }
            }
        });
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct List {
    items: Vec<String>,
    selected: Option<usize>,
    style: Style,
    item_style: Option<Style>,
    selected_style: Option<Style>,
    selected_prefix: String,
    padding: Padding,
    margin: Padding,
}

impl List {
    pub fn new<I, S>(items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            items: items.into_iter().map(Into::into).collect(),
            selected: None,
            style: Style::default(),
            item_style: None,
            selected_style: None,
            selected_prefix: "›".to_string(),
            padding: Padding::default(),
            margin: Padding::default(),
        }
    }

    pub fn selected(mut self, selected: usize) -> Self {
        self.selected = Some(selected);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn item_style(mut self, style: Style) -> Self {
        self.item_style = Some(style);
        self
    }

    pub fn selected_style(mut self, style: Style) -> Self {
        self.selected_style = Some(style);
        self
    }

    pub fn selected_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.selected_prefix = prefix.into();
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn margin(mut self, margin: Padding) -> Self {
        self.margin = margin;
        self
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let area = self.padding.apply(self.margin.apply(area));
        if area.width == 0 || area.height == 0 || self.items.is_empty() {
            return;
        }

        let viewport_height = area.height as usize;
        let selected = self
            .selected
            .unwrap_or(0)
            .min(self.items.len().saturating_sub(1));
        let start = scroll_start(selected, viewport_height, self.items.len());
        let end = (start + viewport_height).min(self.items.len());
        let item_style = self.item_style.unwrap_or(self.style);
        let selected_style = self.selected_style.unwrap_or(item_style);

        frame.render_in(area, |frame| {
            for (row, idx) in (start..end).enumerate() {
                let is_selected = idx == selected;
                let marker = if is_selected {
                    self.selected_prefix.as_str()
                } else {
                    " "
                };
                let mut line = format!("{} {}", marker, self.items[idx]);
                let width = area.width as usize;
                let current_width = line.chars().count();
                if current_width < width {
                    line.push_str(&" ".repeat(width - current_width));
                }
                let clipped: String = line.chars().take(width).collect();
                frame.print_styled(
                    0,
                    row as u16,
                    &clipped,
                    if is_selected {
                        selected_style
                    } else {
                        item_style
                    },
                );
            }
        });
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Tabs {
    labels: Vec<String>,
    selected: usize,
    style: Style,
    active_style: Option<Style>,
    inactive_style: Option<Style>,
    border_style: Option<Style>,
    padding: Padding,
    margin: Padding,
}

impl Tabs {
    pub fn new<I, S>(labels: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            labels: labels.into_iter().map(Into::into).collect(),
            selected: 0,
            style: Style::default(),
            active_style: None,
            inactive_style: None,
            border_style: None,
            padding: Padding::default(),
            margin: Padding::default(),
        }
    }

    pub fn selected(mut self, selected: usize) -> Self {
        self.selected = selected;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn active_style(mut self, style: Style) -> Self {
        self.active_style = Some(style);
        self
    }

    pub fn inactive_style(mut self, style: Style) -> Self {
        self.inactive_style = Some(style);
        self
    }

    pub fn border_style(mut self, style: Style) -> Self {
        self.border_style = Some(style);
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn margin(mut self, margin: Padding) -> Self {
        self.margin = margin;
        self
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let area = self.padding.apply(self.margin.apply(area));
        if area.width == 0 || area.height == 0 || self.labels.is_empty() {
            return;
        }

        let selected = self.selected.min(self.labels.len().saturating_sub(1));
        let base = self.style;
        let active = self.active_style.unwrap_or(base);
        let inactive = self.inactive_style.unwrap_or(base);
        let border = self.border_style.unwrap_or(base);
        let width = area.width as usize;

        frame.render_in(area, |frame| {
            frame.print_styled(0, 0, &" ".repeat(width), base);
            frame.print_styled(0, 0, "[", border);

            let mut cursor = 1usize;
            for (idx, label) in self.labels.iter().enumerate() {
                let tab = format!(" {} ", label);
                let clipped = truncate_to_width(&tab, width.saturating_sub(cursor));
                if clipped.is_empty() {
                    break;
                }
                frame.print_styled(
                    cursor as u16,
                    0,
                    &clipped,
                    if idx == selected { active } else { inactive },
                );
                cursor += clipped.chars().count();
                if cursor >= width.saturating_sub(1) {
                    break;
                }
                frame.print_styled(cursor as u16, 0, "|", border);
                cursor += 1;
            }

            if width > 1 {
                frame.print_styled((width - 1) as u16, 0, "]", border);
            }
        });
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TableColumn {
    pub title: String,
    pub width: Constraint,
    pub align: Alignment,
}

impl TableColumn {
    pub fn new(title: impl Into<String>, width: Constraint) -> Self {
        Self {
            title: title.into(),
            width,
            align: Alignment::Left,
        }
    }

    pub fn align(mut self, align: Alignment) -> Self {
        self.align = align;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Table {
    columns: Vec<TableColumn>,
    rows: Vec<Vec<String>>,
    selected: Option<usize>,
    scroll: Option<usize>,
    style: Style,
    header_style: Option<Style>,
    row_style: Option<Style>,
    selected_style: Option<Style>,
    border_style: Option<Style>,
    padding: Padding,
    margin: Padding,
}

impl Table {
    pub fn new(columns: Vec<TableColumn>, rows: Vec<Vec<String>>) -> Self {
        Self {
            columns,
            rows,
            selected: None,
            scroll: None,
            style: Style::default(),
            header_style: None,
            row_style: None,
            selected_style: None,
            border_style: None,
            padding: Padding::default(),
            margin: Padding::default(),
        }
    }

    pub fn selected(mut self, selected: usize) -> Self {
        self.selected = Some(selected);
        self
    }

    pub fn scroll(mut self, scroll: usize) -> Self {
        self.scroll = Some(scroll);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn header_style(mut self, style: Style) -> Self {
        self.header_style = Some(style);
        self
    }

    pub fn row_style(mut self, style: Style) -> Self {
        self.row_style = Some(style);
        self
    }

    pub fn selected_style(mut self, style: Style) -> Self {
        self.selected_style = Some(style);
        self
    }

    pub fn border_style(mut self, style: Style) -> Self {
        self.border_style = Some(style);
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn margin(mut self, margin: Padding) -> Self {
        self.margin = margin;
        self
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let area = self.padding.apply(self.margin.apply(area));
        if area.width == 0 || area.height == 0 || self.columns.is_empty() {
            return;
        }

        let base = self.style;
        let header_style = self.header_style.unwrap_or(base);
        let row_style = self.row_style.unwrap_or(base);
        let selected_style = self.selected_style.unwrap_or(row_style);
        let border_style = self.border_style.unwrap_or(header_style);

        let column_slots: Vec<Slot> = self
            .columns
            .iter()
            .map(|col| Slot::new(col.width, LayoutNode::leaf("col")))
            .collect();
        let widths = resolve_sizes(area.width, &column_slots);

        frame.render_in(area, |frame| {
            frame.print_styled(0, 0, &" ".repeat(area.width as usize), header_style);
            let mut x = 0u16;
            for (idx, col) in self.columns.iter().enumerate() {
                let width = widths[idx] as usize;
                if width == 0 {
                    continue;
                }
                let header = align_text(&truncate_to_width(&col.title, width), width, col.align);
                frame.print_styled(x, 0, &header, header_style);
                x = x.saturating_add(widths[idx]);
            }

            if area.height > 1 {
                frame.print_styled(0, 1, &"─".repeat(area.width as usize), border_style);
            }

            let body_height = area.height.saturating_sub(2) as usize;
            if body_height == 0 || self.rows.is_empty() {
                return;
            }

            let selected = self
                .selected
                .unwrap_or(0)
                .min(self.rows.len().saturating_sub(1));
            let start = self
                .scroll
                .unwrap_or_else(|| scroll_start(selected, body_height, self.rows.len()));
            let end = (start + body_height).min(self.rows.len());

            for (row_y, row_idx) in (start..end).enumerate() {
                let y = (row_y + 2) as u16;
                frame.print_styled(
                    0,
                    y,
                    &" ".repeat(area.width as usize),
                    if row_idx == selected {
                        selected_style
                    } else {
                        row_style
                    },
                );

                let mut x = 0u16;
                for (col_idx, col) in self.columns.iter().enumerate() {
                    let width = widths[col_idx] as usize;
                    if width == 0 {
                        continue;
                    }
                    let cell = self
                        .rows
                        .get(row_idx)
                        .and_then(|row| row.get(col_idx))
                        .map(|s| s.as_str())
                        .unwrap_or("");
                    let aligned = align_text(&truncate_to_width(cell, width), width, col.align);
                    frame.print_styled(
                        x,
                        y,
                        &aligned,
                        if row_idx == selected {
                            selected_style
                        } else {
                            row_style
                        },
                    );
                    x = x.saturating_add(widths[col_idx]);
                }
            }
        });
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Select {
    options: Vec<String>,
    selected: Option<usize>,
    highlighted: usize,
    expanded: bool,
    placeholder: String,
    max_visible: Option<usize>,
    style: Style,
    selected_style: Option<Style>,
    dropdown_style: Option<Style>,
    highlight_style: Option<Style>,
    padding: Padding,
    margin: Padding,
}

impl Select {
    pub fn new<I, S>(options: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            options: options.into_iter().map(Into::into).collect(),
            selected: None,
            highlighted: 0,
            expanded: false,
            placeholder: String::new(),
            max_visible: None,
            style: Style::default(),
            selected_style: None,
            dropdown_style: None,
            highlight_style: None,
            padding: Padding::default(),
            margin: Padding::default(),
        }
    }

    pub fn selected(mut self, selected: usize) -> Self {
        self.selected = Some(selected);
        self
    }

    pub fn highlighted(mut self, highlighted: usize) -> Self {
        self.highlighted = highlighted;
        self
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn max_visible(mut self, max_visible: usize) -> Self {
        self.max_visible = Some(max_visible.max(1));
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn selected_style(mut self, style: Style) -> Self {
        self.selected_style = Some(style);
        self
    }

    pub fn dropdown_style(mut self, style: Style) -> Self {
        self.dropdown_style = Some(style);
        self
    }

    pub fn highlight_style(mut self, style: Style) -> Self {
        self.highlight_style = Some(style);
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn margin(mut self, margin: Padding) -> Self {
        self.margin = margin;
        self
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let area = self.padding.apply(self.margin.apply(area));
        if area.width == 0 || area.height == 0 {
            return;
        }

        let width = area.width as usize;
        let base_style = self.style;
        let selected_style = self.selected_style.unwrap_or(base_style);
        let dropdown_style = self.dropdown_style.unwrap_or(base_style);
        let highlight_style = self.highlight_style.unwrap_or(selected_style);

        if self.options.is_empty() {
            frame.render_in(area, |frame| {
                let mut row = " ".repeat(width);
                let value = truncate_to_width(&self.placeholder, width.saturating_sub(2));
                replace_segment(&mut row, 0, &value);
                if width > 0 {
                    frame.print_styled(0, 0, &row, base_style);
                    frame.print_styled((width - 1) as u16, 0, "▾", base_style);
                }
            });
            return;
        }

        let selected_idx = self
            .selected
            .map(|idx| idx.min(self.options.len().saturating_sub(1)));
        let highlighted = self.highlighted.min(self.options.len().saturating_sub(1));

        frame.render_in(area, |frame| {
            let mut row = " ".repeat(width);
            let value = selected_idx
                .and_then(|idx| self.options.get(idx))
                .map(|item| item.as_str())
                .filter(|item| !item.is_empty())
                .unwrap_or(self.placeholder.as_str());
            let value = truncate_to_width(value, width.saturating_sub(2));
            replace_segment(&mut row, 0, &value);

            frame.print_styled(0, 0, &row, base_style);
            if !value.is_empty() {
                frame.print_styled(
                    0,
                    0,
                    &value,
                    if selected_idx.is_some() {
                        selected_style
                    } else {
                        base_style
                    },
                );
            }
            if width > 0 {
                frame.print_styled(
                    (width - 1) as u16,
                    0,
                    if self.expanded { "▴" } else { "▾" },
                    base_style,
                );
            }

            if !self.expanded || area.height <= 1 {
                return;
            }

            let viewport = area.height.saturating_sub(1) as usize;
            let max_visible = self.max_visible.unwrap_or(self.options.len());
            let rows = viewport.min(max_visible.max(1));
            let start = scroll_start(highlighted, rows, self.options.len());
            let end = (start + rows).min(self.options.len());

            for (row_idx, option_idx) in (start..end).enumerate() {
                let y = (row_idx + 1) as u16;
                let mut line = " ".repeat(width);
                let pointer = if option_idx == highlighted {
                    "›"
                } else {
                    " "
                };
                let marker = if Some(option_idx) == selected_idx {
                    "●"
                } else {
                    " "
                };
                replace_segment(&mut line, 0, &format!("{pointer}{marker} "));
                let label =
                    truncate_to_width(self.options[option_idx].as_str(), width.saturating_sub(3));
                replace_segment(&mut line, 3, &label);

                let style = if option_idx == highlighted {
                    highlight_style
                } else if Some(option_idx) == selected_idx {
                    selected_style
                } else {
                    dropdown_style
                };

                frame.print_styled(0, y, &line, style);
            }
        });
    }
}

impl Default for Select {
    fn default() -> Self {
        Self::new(Vec::<String>::new())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Checkbox {
    label: String,
    checked: bool,
    focused: bool,
    style: Style,
    checked_style: Option<Style>,
    box_style: Option<Style>,
    focus_style: Option<Style>,
    padding: Padding,
    margin: Padding,
}

impl Checkbox {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            checked: false,
            focused: false,
            style: Style::default(),
            checked_style: None,
            box_style: None,
            focus_style: None,
            padding: Padding::default(),
            margin: Padding::default(),
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn checked_style(mut self, style: Style) -> Self {
        self.checked_style = Some(style);
        self
    }

    pub fn box_style(mut self, style: Style) -> Self {
        self.box_style = Some(style);
        self
    }

    pub fn focus_style(mut self, style: Style) -> Self {
        self.focus_style = Some(style);
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn margin(mut self, margin: Padding) -> Self {
        self.margin = margin;
        self
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let area = self.padding.apply(self.margin.apply(area));
        if area.width == 0 || area.height == 0 {
            return;
        }

        let width = area.width as usize;
        let base_style = self.style;
        let checked_style = self.checked_style.unwrap_or(base_style);
        let box_style = self.box_style.unwrap_or(base_style);
        let focus_style = self.focus_style.unwrap_or(base_style);

        frame.render_in(area, |frame| {
            let mut row = " ".repeat(width);
            let marker = if self.checked { "[x]" } else { "[ ]" };
            let content = format!("{marker} {}", self.label);
            let clipped = truncate_to_width(&content, width);
            replace_segment(&mut row, 0, &clipped);

            frame.print_styled(0, 0, &row, base_style);
            if self.focused {
                frame.print_styled(0, 0, &row, focus_style);
            }

            let marker_chars = marker.chars().count().min(width);
            if marker_chars > 0 {
                frame.print_styled(0, 0, &truncate_to_width(marker, marker_chars), box_style);
            }

            if self.checked {
                frame.print_styled(1, 0, "x", checked_style);
            }
        });
    }
}

impl Default for Checkbox {
    fn default() -> Self {
        Self::new("")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RadioGroup {
    options: Vec<String>,
    selected: Option<usize>,
    highlighted: usize,
    focused: bool,
    max_visible: Option<usize>,
    style: Style,
    selected_style: Option<Style>,
    highlight_style: Option<Style>,
    marker_style: Option<Style>,
    padding: Padding,
    margin: Padding,
}

impl RadioGroup {
    pub fn new<I, S>(options: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            options: options.into_iter().map(Into::into).collect(),
            selected: None,
            highlighted: 0,
            focused: false,
            max_visible: None,
            style: Style::default(),
            selected_style: None,
            highlight_style: None,
            marker_style: None,
            padding: Padding::default(),
            margin: Padding::default(),
        }
    }

    pub fn selected(mut self, selected: usize) -> Self {
        self.selected = Some(selected);
        self
    }

    pub fn highlighted(mut self, highlighted: usize) -> Self {
        self.highlighted = highlighted;
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn max_visible(mut self, max_visible: usize) -> Self {
        self.max_visible = Some(max_visible.max(1));
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn selected_style(mut self, style: Style) -> Self {
        self.selected_style = Some(style);
        self
    }

    pub fn highlight_style(mut self, style: Style) -> Self {
        self.highlight_style = Some(style);
        self
    }

    pub fn marker_style(mut self, style: Style) -> Self {
        self.marker_style = Some(style);
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn margin(mut self, margin: Padding) -> Self {
        self.margin = margin;
        self
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let area = self.padding.apply(self.margin.apply(area));
        if area.width == 0 || area.height == 0 || self.options.is_empty() {
            return;
        }

        let width = area.width as usize;
        let base_style = self.style;
        let selected_style = self.selected_style.unwrap_or(base_style);
        let highlight_style = self.highlight_style.unwrap_or(selected_style);
        let marker_style = self.marker_style.unwrap_or(base_style);

        let selected_idx = self
            .selected
            .map(|idx| idx.min(self.options.len().saturating_sub(1)));
        let highlighted = self.highlighted.min(self.options.len().saturating_sub(1));

        let viewport = area.height as usize;
        let max_visible = self.max_visible.unwrap_or(self.options.len());
        let rows = viewport.min(max_visible.max(1));
        let start = scroll_start(highlighted, rows, self.options.len());
        let end = (start + rows).min(self.options.len());

        frame.render_in(area, |frame| {
            for (row_idx, option_idx) in (start..end).enumerate() {
                let y = row_idx as u16;
                let mut line = " ".repeat(width);
                let is_highlight = self.focused && option_idx == highlighted;
                let pointer = if is_highlight { "›" } else { " " };
                let marker = if Some(option_idx) == selected_idx {
                    "●"
                } else {
                    "○"
                };
                replace_segment(&mut line, 0, &format!("{pointer}{marker} "));
                let label =
                    truncate_to_width(self.options[option_idx].as_str(), width.saturating_sub(3));
                replace_segment(&mut line, 3, &label);

                let style = if is_highlight {
                    highlight_style
                } else if Some(option_idx) == selected_idx {
                    selected_style
                } else {
                    base_style
                };

                frame.print_styled(0, y, &line, style);
                if width > 1 {
                    frame.print_styled(1, y, marker, marker_style);
                }
            }
        });
    }
}

impl Default for RadioGroup {
    fn default() -> Self {
        Self::new(Vec::<String>::new())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Slider {
    min: u16,
    max: u16,
    value: u16,
    focused: bool,
    style: Style,
    track_style: Option<Style>,
    fill_style: Option<Style>,
    thumb_style: Option<Style>,
    focus_style: Option<Style>,
    padding: Padding,
    margin: Padding,
}

impl Slider {
    pub fn new(min: u16, max: u16) -> Self {
        let upper = min.max(max);
        Self {
            min,
            max: upper,
            value: min,
            focused: false,
            style: Style::default(),
            track_style: None,
            fill_style: None,
            thumb_style: None,
            focus_style: None,
            padding: Padding::default(),
            margin: Padding::default(),
        }
    }

    pub fn value(mut self, value: u16) -> Self {
        self.value = value.clamp(self.min, self.max);
        self
    }

    pub fn step(self, step: u16) -> Self {
        let _ = step;
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn track_style(mut self, style: Style) -> Self {
        self.track_style = Some(style);
        self
    }

    pub fn fill_style(mut self, style: Style) -> Self {
        self.fill_style = Some(style);
        self
    }

    pub fn thumb_style(mut self, style: Style) -> Self {
        self.thumb_style = Some(style);
        self
    }

    pub fn focus_style(mut self, style: Style) -> Self {
        self.focus_style = Some(style);
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn margin(mut self, margin: Padding) -> Self {
        self.margin = margin;
        self
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let area = self.padding.apply(self.margin.apply(area));
        if area.width == 0 || area.height == 0 {
            return;
        }

        let width = area.width as usize;
        let base_style = self.style;
        let track_style = self.track_style.unwrap_or(base_style);
        let fill_style = self.fill_style.unwrap_or(track_style);
        let thumb_style = self.thumb_style.unwrap_or(fill_style);
        let focus_style = self.focus_style.unwrap_or(base_style);

        let range = self.max.saturating_sub(self.min) as usize;
        let value = self.value.clamp(self.min, self.max) as usize;
        let rel = value.saturating_sub(self.min as usize);
        let thumb_pos = if width <= 1 || range == 0 {
            0
        } else {
            (rel * (width - 1)) / range
        };

        frame.render_in(area, |frame| {
            let row = " ".repeat(width);
            frame.print_styled(0, 0, &row, base_style);
            if self.focused {
                frame.print_styled(0, 0, &row, focus_style);
            }

            let track = "─".repeat(width);
            frame.print_styled(0, 0, &track, track_style);

            let fill = "━".repeat(thumb_pos.saturating_add(1));
            frame.print_styled(0, 0, &fill, fill_style);
            frame.print_styled(thumb_pos as u16, 0, "●", thumb_style);
        });
    }
}

impl Default for Slider {
    fn default() -> Self {
        Self::new(0, 100)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Switch {
    on: bool,
    focused: bool,
    style: Style,
    on_style: Option<Style>,
    off_style: Option<Style>,
    thumb_style: Option<Style>,
    focus_style: Option<Style>,
    padding: Padding,
    margin: Padding,
}

impl Switch {
    pub fn new() -> Self {
        Self {
            on: false,
            focused: false,
            style: Style::default(),
            on_style: None,
            off_style: None,
            thumb_style: None,
            focus_style: None,
            padding: Padding::default(),
            margin: Padding::default(),
        }
    }

    pub fn on(mut self, on: bool) -> Self {
        self.on = on;
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn on_style(mut self, style: Style) -> Self {
        self.on_style = Some(style);
        self
    }

    pub fn off_style(mut self, style: Style) -> Self {
        self.off_style = Some(style);
        self
    }

    pub fn thumb_style(mut self, style: Style) -> Self {
        self.thumb_style = Some(style);
        self
    }

    pub fn focus_style(mut self, style: Style) -> Self {
        self.focus_style = Some(style);
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn margin(mut self, margin: Padding) -> Self {
        self.margin = margin;
        self
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let area = self.padding.apply(self.margin.apply(area));
        if area.width == 0 || area.height == 0 {
            return;
        }

        let width = area.width as usize;
        let base_style = self.style;
        let on_style = self.on_style.unwrap_or(base_style);
        let off_style = self.off_style.unwrap_or(base_style);
        let thumb_style = self.thumb_style.unwrap_or(base_style);
        let focus_style = self.focus_style.unwrap_or(base_style);

        frame.render_in(area, |frame| {
            let row = " ".repeat(width);
            frame.print_styled(0, 0, &row, base_style);
            if self.focused {
                frame.print_styled(0, 0, &row, focus_style);
            }

            let track = if self.on { "[ ON ]" } else { "[OFF ]" };
            let clipped = truncate_to_width(track, width);
            frame.print_styled(0, 0, &clipped, if self.on { on_style } else { off_style });

            let thumb_x = if self.on { 4 } else { 1 };
            if width > thumb_x {
                frame.print_styled(thumb_x as u16, 0, "●", thumb_style);
            }
        });
    }
}

impl Default for Switch {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Stepper {
    min: u16,
    max: u16,
    value: u16,
    step: u16,
    focused: bool,
    style: Style,
    value_style: Option<Style>,
    controls_style: Option<Style>,
    focus_style: Option<Style>,
    padding: Padding,
    margin: Padding,
}

impl Stepper {
    pub fn new(min: u16, max: u16) -> Self {
        let upper = min.max(max);
        Self {
            min,
            max: upper,
            value: min,
            step: 1,
            focused: false,
            style: Style::default(),
            value_style: None,
            controls_style: None,
            focus_style: None,
            padding: Padding::default(),
            margin: Padding::default(),
        }
    }

    pub fn value(mut self, value: u16) -> Self {
        self.value = value.clamp(self.min, self.max);
        self
    }

    pub fn step(mut self, step: u16) -> Self {
        self.step = step.max(1);
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn value_style(mut self, style: Style) -> Self {
        self.value_style = Some(style);
        self
    }

    pub fn controls_style(mut self, style: Style) -> Self {
        self.controls_style = Some(style);
        self
    }

    pub fn focus_style(mut self, style: Style) -> Self {
        self.focus_style = Some(style);
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn margin(mut self, margin: Padding) -> Self {
        self.margin = margin;
        self
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let area = self.padding.apply(self.margin.apply(area));
        if area.width == 0 || area.height == 0 {
            return;
        }

        let width = area.width as usize;
        let base_style = self.style;
        let value_style = self.value_style.unwrap_or(base_style);
        let controls_style = self.controls_style.unwrap_or(base_style);
        let focus_style = self.focus_style.unwrap_or(base_style);
        let _step = self.step;

        let value = self.value.clamp(self.min, self.max);
        let value_width = self.max.max(self.min).to_string().len().max(1);
        let content = format!("[-] {:>width$} [+]", value, width = value_width);
        let clipped = truncate_to_width(&content, width);
        let value_start = 4;

        frame.render_in(area, |frame| {
            let row = " ".repeat(width);
            frame.print_styled(0, 0, &row, base_style);
            if self.focused {
                frame.print_styled(0, 0, &row, focus_style);
            }

            frame.print_styled(0, 0, &clipped, base_style);
            if width >= 3 {
                frame.print_styled(0, 0, "[-]", controls_style);
            }
            if width > value_start {
                let value_text = truncate_to_width(value.to_string().as_str(), value_width);
                frame.print_styled(value_start as u16, 0, &value_text, value_style);
            }
            if width > value_start + value_width + 1 {
                let plus_start = value_start + value_width + 1;
                let plus = truncate_to_width("[+]", width.saturating_sub(plus_start));
                frame.print_styled(plus_start as u16, 0, &plus, controls_style);
            }
        });
    }
}

impl Default for Stepper {
    fn default() -> Self {
        Self::new(0, 10)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProgressBar {
    value: u16,
    max: u16,
    show_label: bool,
    style: Style,
    track_style: Option<Style>,
    fill_style: Option<Style>,
    label_style: Option<Style>,
    padding: Padding,
    margin: Padding,
}

impl ProgressBar {
    pub fn new() -> Self {
        Self {
            value: 0,
            max: 100,
            show_label: true,
            style: Style::default(),
            track_style: None,
            fill_style: None,
            label_style: None,
            padding: Padding::default(),
            margin: Padding::default(),
        }
    }

    pub fn value(mut self, value: u16) -> Self {
        self.value = value;
        self
    }

    pub fn max(mut self, max: u16) -> Self {
        self.max = max.max(1);
        self
    }

    pub fn show_label(mut self, show_label: bool) -> Self {
        self.show_label = show_label;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn track_style(mut self, style: Style) -> Self {
        self.track_style = Some(style);
        self
    }

    pub fn fill_style(mut self, style: Style) -> Self {
        self.fill_style = Some(style);
        self
    }

    pub fn label_style(mut self, style: Style) -> Self {
        self.label_style = Some(style);
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn margin(mut self, margin: Padding) -> Self {
        self.margin = margin;
        self
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let area = self.padding.apply(self.margin.apply(area));
        if area.width == 0 || area.height == 0 {
            return;
        }

        let width = area.width as usize;
        let base_style = self.style;
        let track_style = self.track_style.unwrap_or(base_style);
        let fill_style = self.fill_style.unwrap_or(track_style);
        let label_style = self.label_style.unwrap_or(base_style);

        let max = self.max.max(1) as usize;
        let value = self.value.min(self.max) as usize;
        let filled = (value * width) / max;

        frame.render_in(area, |frame| {
            let track = "░".repeat(width);
            frame.print_styled(0, 0, &track, track_style);
            if filled > 0 {
                let fill = "█".repeat(filled);
                frame.print_styled(0, 0, &fill, fill_style);
            }

            if self.show_label {
                let percent = (value * 100) / max;
                let label = format!("{percent:>3}%");
                let clipped = truncate_to_width(&label, width);
                let x = width.saturating_sub(clipped.chars().count());
                frame.print_styled(x as u16, 0, &clipped, label_style);
            }
        });
    }
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultiSelect {
    options: Vec<String>,
    selected: Vec<usize>,
    highlighted: usize,
    focused: bool,
    max_visible: Option<usize>,
    style: Style,
    selected_style: Option<Style>,
    highlight_style: Option<Style>,
    marker_style: Option<Style>,
    padding: Padding,
    margin: Padding,
}

impl MultiSelect {
    pub fn new<I, S>(options: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            options: options.into_iter().map(Into::into).collect(),
            selected: Vec::new(),
            highlighted: 0,
            focused: false,
            max_visible: None,
            style: Style::default(),
            selected_style: None,
            highlight_style: None,
            marker_style: None,
            padding: Padding::default(),
            margin: Padding::default(),
        }
    }

    pub fn selected(mut self, selected: Vec<usize>) -> Self {
        self.selected = selected;
        self
    }

    pub fn highlighted(mut self, highlighted: usize) -> Self {
        self.highlighted = highlighted;
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn max_visible(mut self, max_visible: usize) -> Self {
        self.max_visible = Some(max_visible.max(1));
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn selected_style(mut self, style: Style) -> Self {
        self.selected_style = Some(style);
        self
    }

    pub fn highlight_style(mut self, style: Style) -> Self {
        self.highlight_style = Some(style);
        self
    }

    pub fn marker_style(mut self, style: Style) -> Self {
        self.marker_style = Some(style);
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn margin(mut self, margin: Padding) -> Self {
        self.margin = margin;
        self
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let area = self.padding.apply(self.margin.apply(area));
        if area.width == 0 || area.height == 0 || self.options.is_empty() {
            return;
        }

        let width = area.width as usize;
        let base_style = self.style;
        let selected_style = self.selected_style.unwrap_or(base_style);
        let highlight_style = self.highlight_style.unwrap_or(selected_style);
        let marker_style = self.marker_style.unwrap_or(base_style);
        let highlighted = self.highlighted.min(self.options.len().saturating_sub(1));

        let viewport = area.height as usize;
        let max_visible = self.max_visible.unwrap_or(self.options.len());
        let rows = viewport.min(max_visible.max(1));
        let start = scroll_start(highlighted, rows, self.options.len());
        let end = (start + rows).min(self.options.len());

        frame.render_in(area, |frame| {
            for (row_idx, option_idx) in (start..end).enumerate() {
                let y = row_idx as u16;
                let mut line = " ".repeat(width);
                let is_selected = self.selected.contains(&option_idx);
                let is_highlight = self.focused && option_idx == highlighted;
                let pointer = if is_highlight { "›" } else { " " };
                let marker = if is_selected { "x" } else { " " };
                replace_segment(&mut line, 0, &format!("{pointer}[{marker}] "));
                let label =
                    truncate_to_width(self.options[option_idx].as_str(), width.saturating_sub(5));
                replace_segment(&mut line, 5, &label);

                let style = if is_highlight {
                    highlight_style
                } else if is_selected {
                    selected_style
                } else {
                    base_style
                };
                frame.print_styled(0, y, &line, style);
                if width > 2 {
                    frame.print_styled(1, y, "[", marker_style);
                    frame.print_styled(2, y, marker, marker_style);
                    frame.print_styled(3, y, "]", marker_style);
                }
            }
        });
    }
}

impl Default for MultiSelect {
    fn default() -> Self {
        Self::new(Vec::<String>::new())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormField {
    label: String,
    help_text: Option<String>,
    error_text: Option<String>,
    style: Style,
    label_style: Option<Style>,
    help_style: Option<Style>,
    error_style: Option<Style>,
    padding: Padding,
    margin: Padding,
}

impl FormField {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            help_text: None,
            error_text: None,
            style: Style::default(),
            label_style: None,
            help_style: None,
            error_style: None,
            padding: Padding::default(),
            margin: Padding::default(),
        }
    }

    pub fn help_text(mut self, text: impl Into<String>) -> Self {
        self.help_text = Some(text.into());
        self
    }

    pub fn error_text(mut self, text: impl Into<String>) -> Self {
        self.error_text = Some(text.into());
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn label_style(mut self, style: Style) -> Self {
        self.label_style = Some(style);
        self
    }

    pub fn help_style(mut self, style: Style) -> Self {
        self.help_style = Some(style);
        self
    }

    pub fn error_style(mut self, style: Style) -> Self {
        self.error_style = Some(style);
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn margin(mut self, margin: Padding) -> Self {
        self.margin = margin;
        self
    }

    pub fn render(
        &self,
        frame: &mut Frame,
        area: Rect,
        render_control: impl FnOnce(&mut Frame, Rect),
    ) {
        let area = self.padding.apply(self.margin.apply(area));
        if area.width == 0 || area.height == 0 {
            return;
        }

        let label_style = self.label_style.unwrap_or(self.style);
        let help_style = self.help_style.unwrap_or(self.style);
        let error_style = self.error_style.unwrap_or(self.style);

        frame.render_in(area, |frame| {
            frame.print_styled(
                0,
                0,
                &truncate_to_width(&self.label, area.width as usize),
                label_style,
            );

            let footer = self.error_text.as_ref().or(self.help_text.as_ref());
            let footer_style = if self.error_text.is_some() {
                error_style
            } else {
                help_style
            };
            let footer_rows = u16::from(footer.is_some());
            let control_y = 1;
            let control_h = area.height.saturating_sub(1).saturating_sub(footer_rows);
            if control_h > 0 {
                render_control(frame, Rect::new(0, control_y, area.width, control_h));
            }

            if let Some(footer_text) = footer {
                let y = area.height.saturating_sub(1);
                frame.print_styled(
                    0,
                    y,
                    &truncate_to_width(footer_text, area.width as usize),
                    footer_style,
                );
            }
        });
    }
}

#[derive(Clone, Copy)]
struct BorderGlyphs {
    horizontal: &'static str,
    vertical: &'static str,
    top_left: &'static str,
    top_right: &'static str,
    bottom_left: &'static str,
    bottom_right: &'static str,
}

fn border_glyphs(border_type: BorderType) -> BorderGlyphs {
    match border_type {
        BorderType::Unicode => BorderGlyphs {
            horizontal: "─",
            vertical: "│",
            top_left: "┌",
            top_right: "┐",
            bottom_left: "└",
            bottom_right: "┘",
        },
        BorderType::Ascii => BorderGlyphs {
            horizontal: "-",
            vertical: "|",
            top_left: "+",
            top_right: "+",
            bottom_left: "+",
            bottom_right: "+",
        },
    }
}

fn fill_with_style(frame: &mut Frame, area: Rect, style: Style) {
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

fn wrap_lines(text: &str, width: usize, wrap: WrapMode) -> Vec<String> {
    if width == 0 {
        return Vec::new();
    }

    match wrap {
        WrapMode::NoWrap => text
            .lines()
            .map(|line| truncate_to_width(line, width))
            .collect(),
        WrapMode::Char => {
            let mut out = Vec::new();
            for line in text.lines() {
                let chars: Vec<char> = line.chars().collect();
                if chars.is_empty() {
                    out.push(String::new());
                    continue;
                }
                let mut start = 0;
                while start < chars.len() {
                    let end = (start + width).min(chars.len());
                    out.push(chars[start..end].iter().collect());
                    start = end;
                }
            }
            out
        }
        WrapMode::Word => {
            let mut out = Vec::new();
            for source in text.lines() {
                if source.is_empty() {
                    out.push(String::new());
                    continue;
                }

                let mut current = String::new();
                for word in source.split_whitespace() {
                    let word_len = word.chars().count();
                    if word_len > width {
                        if !current.is_empty() {
                            out.push(current.clone());
                            current.clear();
                        }
                        out.extend(wrap_lines(word, width, WrapMode::Char));
                        continue;
                    }

                    let sep = if current.is_empty() { 0 } else { 1 };
                    if current.chars().count() + sep + word_len > width {
                        out.push(current.clone());
                        current.clear();
                    }

                    if !current.is_empty() {
                        current.push(' ');
                    }
                    current.push_str(word);
                }

                if !current.is_empty() {
                    out.push(current);
                }
            }
            out
        }
    }
}

fn truncate_to_width(input: &str, width: usize) -> String {
    input.chars().take(width).collect()
}

fn align_text(text: &str, width: usize, align: Alignment) -> String {
    let text_width = text.chars().count().min(width);
    let free = width.saturating_sub(text_width);
    let (left, right) = match align {
        Alignment::Left => (0, free),
        Alignment::Center => (free / 2, free - (free / 2)),
        Alignment::Right => (free, 0),
    };

    format!(
        "{}{}{}",
        " ".repeat(left),
        truncate_to_width(text, width),
        " ".repeat(right)
    )
}

fn replace_segment(target: &mut String, start: usize, segment: &str) {
    let mut chars: Vec<char> = target.chars().collect();
    for (offset, ch) in segment.chars().enumerate() {
        let idx = start + offset;
        if idx >= chars.len() {
            break;
        }
        chars[idx] = ch;
    }
    *target = chars.into_iter().collect();
}

fn scroll_start(selected: usize, viewport_height: usize, len: usize) -> usize {
    if viewport_height == 0 || len == 0 {
        return 0;
    }

    let max_start = len.saturating_sub(viewport_height);
    let follow = selected.saturating_add(1).saturating_sub(viewport_height);
    follow.min(max_start)
}

fn resolve_node(node: &LayoutNode, area: Rect, zones: &mut Vec<Zone>) {
    let area = node.padding.apply(area);
    zones.push(Zone {
        name: node.name.clone(),
        area,
    });

    let NodeKind::Split {
        direction,
        children,
    } = &node.kind
    else {
        return;
    };

    let pieces = split_area(area, *direction, children);
    for (slot, child_area) in children.iter().zip(pieces.into_iter()) {
        resolve_node(&slot.node, child_area, zones);
    }
}

fn split_area(area: Rect, direction: Direction, children: &[Slot]) -> Vec<Rect> {
    if children.is_empty() {
        return Vec::new();
    }

    let total = match direction {
        Direction::Horizontal => area.width,
        Direction::Vertical => area.height,
    };

    let sizes = resolve_sizes(total, children);
    let mut cursor_x = area.x;
    let mut cursor_y = area.y;
    let mut pieces = Vec::with_capacity(children.len());

    for size in sizes {
        let rect = match direction {
            Direction::Horizontal => {
                let rect = Rect::new(cursor_x, area.y, size, area.height);
                cursor_x = cursor_x.saturating_add(size);
                rect
            }
            Direction::Vertical => {
                let rect = Rect::new(area.x, cursor_y, area.width, size);
                cursor_y = cursor_y.saturating_add(size);
                rect
            }
        };
        pieces.push(rect);
    }

    pieces
}

fn resolve_sizes(total: u16, children: &[Slot]) -> Vec<u16> {
    let mut sizes = vec![0_u16; children.len()];
    let mut remaining = total;

    for (idx, slot) in children.iter().enumerate() {
        if let Constraint::Fixed(value) = slot.constraint {
            let size = value.min(remaining);
            sizes[idx] = size;
            remaining = remaining.saturating_sub(size);
        }
    }

    for (idx, slot) in children.iter().enumerate() {
        if let Constraint::Percent(value) = slot.constraint {
            let requested = ((total as u32 * value as u32) / 100) as u16;
            let size = requested.min(remaining);
            sizes[idx] = size;
            remaining = remaining.saturating_sub(size);
        }
    }

    let fill_indices: Vec<usize> = children
        .iter()
        .enumerate()
        .filter_map(|(idx, slot)| matches!(slot.constraint, Constraint::Fill).then_some(idx))
        .collect();

    if !fill_indices.is_empty() {
        let share = remaining / fill_indices.len() as u16;
        let mut extra = remaining % fill_indices.len() as u16;

        for idx in fill_indices {
            let mut size = share;
            if extra > 0 {
                size = size.saturating_add(1);
                extra -= 1;
            }
            sizes[idx] = size;
        }
        remaining = 0;
    }

    if remaining > 0 {
        if let Some(last) = sizes.last_mut() {
            *last = last.saturating_add(remaining);
        }
    }

    sizes
}

#[cfg(test)]
mod tests {
    use crate::Frame;

    use super::{
        apply_input_edit, Alignment, Block, BorderType, Borders, Checkbox, CheckboxStyle,
        Constraint, Direction, FormField, FormFieldStyle, Input, InputEdit, InputStyle, LayoutNode,
        List, ListStyle, MultiSelect, MultiSelectStyle, Padding, Panel, PanelStyle, Paragraph,
        ProgressBar, ProgressBarStyle, RadioGroup, RadioGroupStyle, Select, SelectStyle, Slider,
        SliderStyle, Slot, StatusBar, StatusBarStyle, Stepper, StepperStyle, Switch, SwitchStyle,
        Table, TableColumn, TableStyle, Tabs, TabsStyle, Text, WrapMode,
    };
    use crate::{Color, Rect, Style, Theme};

    #[test]
    fn resolve_mixed_constraints_and_preserve_width() {
        let layout = LayoutNode::split(
            "root",
            Direction::Horizontal,
            [
                Slot::new(Constraint::Fixed(10), LayoutNode::leaf("a")),
                Slot::new(Constraint::Percent(25), LayoutNode::leaf("b")),
                Slot::new(Constraint::Fill, LayoutNode::leaf("c")),
                Slot::new(Constraint::Fill, LayoutNode::leaf("d")),
            ],
        );

        let resolved = layout.resolve(Rect::new(0, 0, 100, 10));

        assert_eq!(resolved.area("a"), Some(Rect::new(0, 0, 10, 10)));
        assert_eq!(resolved.area("b"), Some(Rect::new(10, 0, 25, 10)));
        assert_eq!(resolved.area("c"), Some(Rect::new(35, 0, 33, 10)));
        assert_eq!(resolved.area("d"), Some(Rect::new(68, 0, 32, 10)));
    }

    #[test]
    fn resolve_clamps_when_constraints_overflow() {
        let layout = LayoutNode::split(
            "root",
            Direction::Horizontal,
            [
                Slot::new(Constraint::Fixed(15), LayoutNode::leaf("a")),
                Slot::new(Constraint::Percent(50), LayoutNode::leaf("b")),
                Slot::new(Constraint::Fill, LayoutNode::leaf("c")),
            ],
        );

        let resolved = layout.resolve(Rect::new(0, 0, 20, 5));

        assert_eq!(resolved.area("a"), Some(Rect::new(0, 0, 15, 5)));
        assert_eq!(resolved.area("b"), Some(Rect::new(15, 0, 5, 5)));
        assert_eq!(resolved.area("c"), Some(Rect::new(20, 0, 0, 5)));
    }

    #[test]
    fn padding_applies_safely_with_saturation() {
        let area = Rect::new(0, 0, 1, 1);
        let padded = Padding::all(2).apply(area);

        assert_eq!(padded, Rect::new(1, 1, 0, 0));
    }

    #[test]
    fn resolve_applies_padding_on_nested_nodes() {
        let layout = LayoutNode::split(
            "root",
            Direction::Vertical,
            [Slot::new(
                Constraint::Fill,
                LayoutNode::leaf("content").with_padding(Padding::symmetric(1, 2)),
            )],
        );

        let resolved = layout.resolve(Rect::new(0, 0, 20, 8));
        assert_eq!(resolved.area("content"), Some(Rect::new(2, 1, 16, 6)));
    }

    #[test]
    fn text_renders_multiline_inside_area() {
        let mut frame = Frame::new(10, 4);
        let text = Text::new("ab\ncd\nef");

        text.render(&mut frame, Rect::new(2, 1, 2, 2));

        assert_eq!(frame.char_at(2, 1), Some('a'));
        assert_eq!(frame.char_at(3, 1), Some('b'));
        assert_eq!(frame.char_at(2, 2), Some('c'));
        assert_eq!(frame.char_at(3, 2), Some('d'));
        assert_eq!(frame.char_at(2, 3), Some(' '));
    }

    #[test]
    fn text_applies_inline_style() {
        let mut frame = Frame::new(6, 2);
        let style = Style::new().fg(Color::Ansi(45)).bg(Color::Ansi(16));
        Text::new("hi")
            .style(style)
            .render(&mut frame, Rect::new(1, 0, 3, 1));

        assert_eq!(frame.style_at(1, 0), Some(style));
        assert_eq!(frame.style_at(2, 0), Some(style));
    }

    #[test]
    fn block_renders_unicode_borders() {
        let mut frame = Frame::new(8, 4);
        let block = Block::new();

        block.render(&mut frame, Rect::new(0, 0, 8, 4));

        assert_eq!(frame.char_at(0, 0), Some('┌'));
        assert_eq!(frame.char_at(7, 0), Some('┐'));
        assert_eq!(frame.char_at(0, 3), Some('└'));
        assert_eq!(frame.char_at(7, 3), Some('┘'));
        assert_eq!(frame.char_at(3, 0), Some('─'));
        assert_eq!(frame.char_at(0, 1), Some('│'));
        assert_eq!(frame.char_at(7, 2), Some('│'));
    }

    #[test]
    fn block_title_is_truncated_to_inner_width() {
        let mut frame = Frame::new(7, 3);
        let block = Block::new().title("abcdef");

        block.render(&mut frame, Rect::new(0, 0, 7, 3));

        assert_eq!(frame.char_at(1, 0), Some(' '));
        assert_eq!(frame.char_at(2, 0), Some('a'));
        assert_eq!(frame.char_at(3, 0), Some('b'));
        assert_eq!(frame.char_at(4, 0), Some('c'));
        assert_eq!(frame.char_at(5, 0), Some('d'));
    }

    #[test]
    fn block_supports_ascii_borders() {
        let mut frame = Frame::new(6, 3);
        let block = Block::new().border_type(BorderType::Ascii);

        block.render(&mut frame, Rect::new(0, 0, 6, 3));

        assert_eq!(frame.char_at(0, 0), Some('+'));
        assert_eq!(frame.char_at(1, 0), Some('-'));
        assert_eq!(frame.char_at(0, 1), Some('|'));
    }

    #[test]
    fn block_margin_and_padding_affect_inner_area() {
        let block = Block::new()
            .margin(Padding::all(1))
            .padding(Padding::all(1));
        let inner = block.inner_area(Rect::new(0, 0, 10, 6));

        assert_eq!(inner, Rect::new(3, 3, 4, 0));
    }

    #[test]
    fn block_without_borders_still_renders_title() {
        let mut frame = Frame::new(8, 2);
        let block = Block::new().title("no-border").borders(Borders::none());

        block.render(&mut frame, Rect::new(0, 0, 8, 2));

        assert_eq!(frame.char_at(0, 0), Some(' '));
        assert_eq!(frame.char_at(1, 0), Some('n'));
    }

    #[test]
    fn list_scrolls_to_keep_selected_visible() {
        let mut frame = Frame::new(12, 3);
        let items = ["zero", "one", "two", "three", "four", "five"];
        let list = List::new(items).selected(4);

        list.render(&mut frame, Rect::new(0, 0, 12, 3));

        assert_eq!(frame.char_at(2, 0), Some('t'));
        assert_eq!(frame.char_at(2, 1), Some('t'));
        assert_eq!(frame.char_at(0, 2), Some('›'));
        assert_eq!(frame.char_at(2, 2), Some('f'));
    }

    #[test]
    fn list_empty_is_noop() {
        let mut frame = Frame::new(4, 2);
        List::new(Vec::<String>::new()).render(&mut frame, Rect::new(0, 0, 4, 2));

        for y in 0..frame.height() {
            for x in 0..frame.width() {
                assert_eq!(frame.char_at(x, y), Some(' '));
            }
        }
    }

    #[test]
    fn list_applies_selected_style_and_prefix() {
        let mut frame = Frame::new(8, 2);
        let selected_style = Style::new().bg(Color::Ansi(34));
        let list = List::new(["one", "two"])
            .selected(1)
            .selected_prefix(">")
            .selected_style(selected_style);

        list.render(&mut frame, Rect::new(0, 0, 8, 2));

        assert_eq!(frame.char_at(0, 1), Some('>'));
        assert_eq!(frame.style_at(0, 1), Some(selected_style));
    }

    #[test]
    fn paragraph_wraps_words_and_clips_height() {
        let mut frame = Frame::new(12, 2);
        Paragraph::new("alpha beta gamma")
            .wrap(WrapMode::Word)
            .render(&mut frame, Rect::new(0, 0, 5, 2));

        assert_eq!(frame.char_at(0, 0), Some('a'));
        assert_eq!(frame.char_at(0, 1), Some('b'));
    }

    #[test]
    fn statusbar_places_left_and_right_segments() {
        let mut frame = Frame::new(14, 1);
        StatusBar::new()
            .left("left")
            .right("right")
            .render(&mut frame, Rect::new(0, 0, 14, 1));

        assert_eq!(frame.char_at(0, 0), Some('l'));
        assert_eq!(frame.char_at(9, 0), Some('r'));
    }

    #[test]
    fn statusbar_handles_collision_with_truncation() {
        let mut frame = Frame::new(8, 1);
        StatusBar::new()
            .left("left-side")
            .right("right-side")
            .render(&mut frame, Rect::new(0, 0, 8, 1));

        assert_eq!(frame.char_at(0, 0), Some('l'));
        assert_eq!(frame.char_at(7, 0), Some('d'));
    }

    #[test]
    fn apply_input_edit_updates_value_and_cursor() {
        let mut value = String::from("ab");
        let mut cursor = 2;

        apply_input_edit(&mut value, &mut cursor, InputEdit::Left);
        apply_input_edit(&mut value, &mut cursor, InputEdit::Backspace);
        apply_input_edit(&mut value, &mut cursor, InputEdit::Insert('z'));

        assert_eq!(value, "zb");
        assert_eq!(cursor, 1);
    }

    #[test]
    fn input_renders_placeholder_when_empty() {
        let mut frame = Frame::new(10, 1);
        let style = Style::new().fg(Color::Ansi(245));
        Input::new()
            .placeholder("search")
            .placeholder_style(style)
            .render(&mut frame, Rect::new(0, 0, 10, 1));

        assert_eq!(frame.char_at(0, 0), Some('s'));
        assert_eq!(frame.style_at(0, 0), Some(style));
    }

    #[test]
    fn input_width_one_and_cursor_bounds_are_safe() {
        let mut frame = Frame::new(1, 1);
        Input::new()
            .value("abc")
            .cursor(99)
            .focused(true)
            .render(&mut frame, Rect::new(0, 0, 1, 1));

        assert_eq!(frame.char_at(0, 0), Some('a'));
    }

    #[test]
    fn paragraph_char_and_no_wrap_modes_render_differently() {
        let mut frame = Frame::new(5, 2);
        Paragraph::new("abcdef")
            .wrap(WrapMode::Char)
            .render(&mut frame, Rect::new(0, 0, 3, 2));
        assert_eq!(frame.char_at(0, 1), Some('d'));

        let mut frame2 = Frame::new(5, 2);
        Paragraph::new("abcdef")
            .wrap(WrapMode::NoWrap)
            .render(&mut frame2, Rect::new(0, 0, 3, 2));
        assert_eq!(frame2.char_at(0, 1), Some(' '));
    }

    #[test]
    fn style_bundles_use_theme_tokens() {
        let theme = Theme::from_json_str(
            r#"{
                "tokens": {
                    "panel.title": { "fg": { "ansi": 111 } },
                    "list.selected": { "fg": { "ansi": 222 } },
                    "statusbar.right": { "fg": { "ansi": 123 } },
                    "input.placeholder": { "fg": { "ansi": 77 } }
                }
            }"#,
        )
        .expect("theme should parse");

        let panel = PanelStyle::from_theme(&theme);
        let list = ListStyle::from_theme(&theme);
        let status = StatusBarStyle::from_theme(&theme);
        let input = InputStyle::from_theme(&theme);

        assert_eq!(panel.title.fg, Some(Color::Ansi(111)));
        assert_eq!(list.selected.fg, Some(Color::Ansi(222)));
        assert_eq!(status.right.fg, Some(Color::Ansi(123)));
        assert_eq!(input.placeholder.fg, Some(Color::Ansi(77)));
    }

    #[test]
    fn panel_wrapper_renders_title_and_inner() {
        let mut frame = Frame::new(12, 4);
        let panel = Panel::new("x").styles(PanelStyle {
            body: Style::new(),
            border: Style::new(),
            title: Style::new(),
        });

        panel.render(&mut frame, Rect::new(0, 0, 12, 4), |f, inner| {
            f.render_in(inner, |f| f.print(0, 0, "ok"));
        });

        assert_eq!(frame.char_at(2, 0), Some('x'));
        assert_eq!(frame.char_at(1, 1), Some('o'));
    }

    #[test]
    fn tabs_highlights_selected_tab() {
        let mut frame = Frame::new(20, 1);
        let active = Style::new().bg(Color::Ansi(39));
        Tabs::new(["A", "B", "C"])
            .selected(1)
            .active_style(active)
            .render(&mut frame, Rect::new(0, 0, 20, 1));

        assert_eq!(frame.char_at(6, 0), Some('B'));
        assert_eq!(frame.style_at(6, 0), Some(active));
    }

    #[test]
    fn table_supports_center_and_right_alignment() {
        let mut frame = Frame::new(12, 4);
        let columns = vec![
            TableColumn::new("L", Constraint::Fixed(4)),
            TableColumn::new("C", Constraint::Fixed(4)).align(Alignment::Center),
            TableColumn::new("R", Constraint::Fixed(4)).align(Alignment::Right),
        ];
        let rows = vec![vec!["a".into(), "b".into(), "c".into()]];
        Table::new(columns, rows).render(&mut frame, Rect::new(0, 0, 12, 4));

        assert_eq!(frame.char_at(0, 2), Some('a'));
        assert_eq!(frame.char_at(5, 2), Some('b'));
        assert_eq!(frame.char_at(11, 2), Some('c'));
    }

    #[test]
    fn form_field_renders_label_and_help() {
        let mut frame = Frame::new(20, 4);
        FormField::new("Name").help_text("Required").render(
            &mut frame,
            Rect::new(0, 0, 20, 4),
            |f, area| {
                f.render_in(area, |f| f.print(0, 0, "input"));
            },
        );

        assert_eq!(frame.char_at(0, 0), Some('N'));
        assert_eq!(frame.char_at(0, 1), Some('i'));
        assert_eq!(frame.char_at(0, 3), Some('R'));
    }

    #[test]
    fn select_renders_collapsed_value_and_indicator() {
        let mut frame = Frame::new(14, 1);
        Select::new(["dev", "stage", "prod"])
            .selected(2)
            .render(&mut frame, Rect::new(0, 0, 14, 1));

        assert_eq!(frame.char_at(0, 0), Some('p'));
        assert_eq!(frame.char_at(13, 0), Some('▾'));
    }

    #[test]
    fn select_open_shows_highlight_and_selected_markers() {
        let mut frame = Frame::new(12, 4);
        Select::new(["dev", "stage", "prod"])
            .selected(0)
            .highlighted(1)
            .expanded(true)
            .render(&mut frame, Rect::new(0, 0, 12, 4));

        assert_eq!(frame.char_at(1, 1), Some('●'));
        assert_eq!(frame.char_at(0, 2), Some('›'));
    }

    #[test]
    fn select_scrolls_to_keep_highlight_visible() {
        let mut frame = Frame::new(10, 3);
        Select::new(["item0", "item1", "item2", "item3", "item4"])
            .highlighted(4)
            .expanded(true)
            .render(&mut frame, Rect::new(0, 0, 10, 3));

        assert_eq!(frame.char_at(7, 1), Some('3'));
        assert_eq!(frame.char_at(0, 2), Some('›'));
        assert_eq!(frame.char_at(7, 2), Some('4'));
    }

    #[test]
    fn checkbox_renders_checked_marker() {
        let mut frame = Frame::new(14, 1);
        Checkbox::new("Auto deploy")
            .checked(true)
            .render(&mut frame, Rect::new(0, 0, 14, 1));

        assert_eq!(frame.char_at(0, 0), Some('['));
        assert_eq!(frame.char_at(1, 0), Some('x'));
        assert_eq!(frame.char_at(4, 0), Some('A'));
    }

    #[test]
    fn checkbox_applies_focus_style() {
        let mut frame = Frame::new(10, 1);
        let focus = Style::new().bg(Color::Ansi(111));
        Checkbox::new("A")
            .focused(true)
            .focus_style(focus)
            .render(&mut frame, Rect::new(0, 0, 10, 1));

        assert_eq!(frame.style_at(4, 0), Some(focus));
    }

    #[test]
    fn radio_group_renders_selected_marker() {
        let mut frame = Frame::new(14, 2);
        RadioGroup::new(["rolling", "canary"])
            .selected(1)
            .render(&mut frame, Rect::new(0, 0, 14, 2));

        assert_eq!(frame.char_at(1, 1), Some('●'));
        assert_eq!(frame.char_at(3, 1), Some('c'));
    }

    #[test]
    fn radio_group_uses_highlight_when_focused() {
        let mut frame = Frame::new(12, 2);
        let highlight = Style::new().bg(Color::Ansi(113));
        RadioGroup::new(["a", "b"])
            .highlighted(1)
            .focused(true)
            .highlight_style(highlight)
            .render(&mut frame, Rect::new(0, 0, 12, 2));

        assert_eq!(frame.char_at(0, 1), Some('›'));
        assert_eq!(frame.style_at(0, 1), Some(highlight));
    }

    #[test]
    fn radio_group_scrolls_to_keep_highlight_visible() {
        let mut frame = Frame::new(12, 2);
        RadioGroup::new(["v0", "v1", "v2", "v3", "v4"])
            .highlighted(4)
            .focused(true)
            .render(&mut frame, Rect::new(0, 0, 12, 2));

        assert_eq!(frame.char_at(4, 0), Some('3'));
        assert_eq!(frame.char_at(0, 1), Some('›'));
        assert_eq!(frame.char_at(4, 1), Some('4'));
    }

    #[test]
    fn slider_places_thumb_from_value() {
        let mut frame = Frame::new(10, 1);
        Slider::new(0, 100)
            .value(50)
            .render(&mut frame, Rect::new(0, 0, 10, 1));

        assert_eq!(frame.char_at(4, 0), Some('●'));
    }

    #[test]
    fn slider_clamps_value_to_bounds() {
        let mut frame = Frame::new(8, 1);
        Slider::new(0, 10)
            .value(99)
            .render(&mut frame, Rect::new(0, 0, 8, 1));

        assert_eq!(frame.char_at(7, 0), Some('●'));
    }

    #[test]
    fn switch_renders_on_and_off_positions() {
        let mut frame = Frame::new(8, 1);
        Switch::new()
            .on(false)
            .render(&mut frame, Rect::new(0, 0, 8, 1));
        assert_eq!(frame.char_at(1, 0), Some('●'));

        let mut frame2 = Frame::new(8, 1);
        Switch::new()
            .on(true)
            .render(&mut frame2, Rect::new(0, 0, 8, 1));
        assert_eq!(frame2.char_at(4, 0), Some('●'));
    }

    #[test]
    fn switch_applies_focus_style() {
        let mut frame = Frame::new(8, 1);
        let focus = Style::new().bg(Color::Ansi(111));
        Switch::new()
            .focused(true)
            .focus_style(focus)
            .render(&mut frame, Rect::new(0, 0, 8, 1));

        assert_eq!(frame.style_at(7, 0), Some(focus));
    }

    #[test]
    fn stepper_renders_value_and_controls() {
        let mut frame = Frame::new(14, 1);
        Stepper::new(0, 10)
            .value(7)
            .render(&mut frame, Rect::new(0, 0, 14, 1));

        assert_eq!(frame.char_at(0, 0), Some('['));
        assert_eq!(frame.char_at(1, 0), Some('-'));
        assert_eq!(frame.char_at(4, 0), Some('7'));
    }

    #[test]
    fn stepper_clamps_value_to_max() {
        let mut frame = Frame::new(14, 1);
        Stepper::new(0, 10)
            .value(99)
            .render(&mut frame, Rect::new(0, 0, 14, 1));

        assert_eq!(frame.char_at(4, 0), Some('1'));
        assert_eq!(frame.char_at(5, 0), Some('0'));
    }

    #[test]
    fn stepper_applies_focus_style() {
        let mut frame = Frame::new(12, 1);
        let focus = Style::new().bg(Color::Ansi(111));
        Stepper::new(0, 10)
            .focused(true)
            .focus_style(focus)
            .render(&mut frame, Rect::new(0, 0, 12, 1));

        assert_eq!(frame.style_at(11, 0), Some(focus));
    }

    #[test]
    fn progress_bar_renders_fill_ratio() {
        let mut frame = Frame::new(10, 1);
        ProgressBar::new()
            .value(50)
            .max(100)
            .show_label(false)
            .render(&mut frame, Rect::new(0, 0, 10, 1));

        assert_eq!(frame.char_at(4, 0), Some('█'));
        assert_eq!(frame.char_at(5, 0), Some('░'));
    }

    #[test]
    fn progress_bar_shows_percentage_label() {
        let mut frame = Frame::new(8, 1);
        ProgressBar::new()
            .value(75)
            .max(100)
            .render(&mut frame, Rect::new(0, 0, 8, 1));

        assert_eq!(frame.char_at(5, 0), Some('7'));
        assert_eq!(frame.char_at(7, 0), Some('%'));
    }

    #[test]
    fn multiselect_marks_selected_items() {
        let mut frame = Frame::new(14, 2);
        MultiSelect::new(["a", "b"])
            .selected(vec![1])
            .render(&mut frame, Rect::new(0, 0, 14, 2));

        assert_eq!(frame.char_at(2, 1), Some('x'));
    }

    #[test]
    fn multiselect_uses_highlight_when_focused() {
        let mut frame = Frame::new(12, 2);
        let highlight = Style::new().bg(Color::Ansi(113));
        MultiSelect::new(["a", "b"])
            .highlighted(1)
            .focused(true)
            .highlight_style(highlight)
            .render(&mut frame, Rect::new(0, 0, 12, 2));

        assert_eq!(frame.char_at(0, 1), Some('›'));
        assert_eq!(frame.style_at(0, 1), Some(highlight));
    }

    #[test]
    fn advanced_style_bundles_use_theme_tokens() {
        let theme = Theme::from_json_str(
            r#"{
              "tokens": {
                "tabs.active": { "fg": { "ansi": 10 } },
                "table.header": { "fg": { "ansi": 11 } },
                "field.error": { "fg": { "ansi": 9 } },
                "select.highlight": { "fg": { "ansi": 208 } },
                "checkbox.checked": { "fg": { "ansi": 12 } },
                "radio.marker": { "fg": { "ansi": 13 } },
                "slider.thumb": { "fg": { "ansi": 14 } },
                "switch.thumb": { "fg": { "ansi": 15 } },
                "stepper.controls": { "fg": { "ansi": 81 } },
                "progress.fill": { "fg": { "ansi": 118 } },
                "multiselect.marker": { "fg": { "ansi": 177 } }
              }
            }"#,
        )
        .expect("theme should parse");

        let tabs = TabsStyle::from_theme(&theme);
        let table = TableStyle::from_theme(&theme);
        let field = FormFieldStyle::from_theme(&theme);
        let select = SelectStyle::from_theme(&theme);
        let checkbox = CheckboxStyle::from_theme(&theme);
        let radio = RadioGroupStyle::from_theme(&theme);
        let slider = SliderStyle::from_theme(&theme);
        let switch = SwitchStyle::from_theme(&theme);
        let stepper = StepperStyle::from_theme(&theme);
        let progress = ProgressBarStyle::from_theme(&theme);
        let multiselect = MultiSelectStyle::from_theme(&theme);

        assert_eq!(tabs.active.fg, Some(Color::Ansi(10)));
        assert_eq!(table.header.fg, Some(Color::Ansi(11)));
        assert_eq!(field.error.fg, Some(Color::Ansi(9)));
        assert_eq!(select.highlight.fg, Some(Color::Ansi(208)));
        assert_eq!(checkbox.checked.fg, Some(Color::Ansi(12)));
        assert_eq!(radio.marker.fg, Some(Color::Ansi(13)));
        assert_eq!(slider.thumb.fg, Some(Color::Ansi(14)));
        assert_eq!(switch.thumb.fg, Some(Color::Ansi(15)));
        assert_eq!(stepper.controls.fg, Some(Color::Ansi(81)));
        assert_eq!(progress.fill.fg, Some(Color::Ansi(118)));
        assert_eq!(multiselect.marker.fg, Some(Color::Ansi(177)));
    }
}
