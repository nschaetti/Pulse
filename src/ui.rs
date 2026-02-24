use crate::{Frame, Rect, Style};

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
        Block, BorderType, Borders, Constraint, Direction, LayoutNode, List, Padding, Slot, Text,
    };
    use crate::{Color, Rect, Style};

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
}
