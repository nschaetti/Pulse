use crate::{Rect, Style};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cell {
    pub ch: char,
    pub style: Style,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            style: Style::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Frame {
    width: u16,
    height: u16,
    cells: Vec<Cell>,
    clip: Rect,
    origin_x: u16,
    origin_y: u16,
}

impl Frame {
    pub fn new(width: u16, height: u16) -> Self {
        let len = width as usize * height as usize;
        Self {
            width,
            height,
            cells: vec![Cell::default(); len],
            clip: Rect::new(0, 0, width, height),
            origin_x: 0,
            origin_y: 0,
        }
    }

    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            cell.ch = ' ';
            cell.style = Style::default();
        }
    }

    pub fn print(&mut self, x: u16, y: u16, text: &str) {
        self.print_styled(x, y, text, Style::default());
    }

    pub fn print_styled(&mut self, x: u16, y: u16, text: &str, style: Style) {
        if self.width == 0 || self.height == 0 {
            return;
        }

        let clip = self.clip;
        if clip.width == 0 || clip.height == 0 {
            return;
        }

        let global_x = self.origin_x as u32 + x as u32;
        let global_y = self.origin_y as u32 + y as u32;

        if global_y >= self.height as u32 {
            return;
        }

        let clip_top = clip.y as u32;
        let clip_bottom = clip_top + clip.height as u32;
        if global_y < clip_top || global_y >= clip_bottom {
            return;
        }

        let clip_left = clip.x as u32;
        let clip_right = clip_left + clip.width as u32;
        let frame_width = self.width as u32;

        for (i, ch) in text.chars().enumerate() {
            let px = global_x + i as u32;
            if px >= frame_width {
                break;
            }

            if px < clip_left || px >= clip_right {
                continue;
            }

            let idx = self.index(px as u16, global_y as u16);
            self.cells[idx].ch = ch;
            self.cells[idx].style = style;
        }
    }

    pub fn render_in(&mut self, area: Rect, f: impl FnOnce(&mut Frame)) {
        let local_area = Rect::new(
            self.origin_x.saturating_add(area.x),
            self.origin_y.saturating_add(area.y),
            area.width,
            area.height,
        );
        let bounds = Rect::new(0, 0, self.width, self.height);
        let next_clip = intersect_rects(intersect_rects(self.clip, local_area), bounds);

        let previous_clip = self.clip;
        let previous_origin_x = self.origin_x;
        let previous_origin_y = self.origin_y;

        self.clip = next_clip;
        self.origin_x = local_area.x;
        self.origin_y = local_area.y;

        f(self);

        self.clip = previous_clip;
        self.origin_x = previous_origin_x;
        self.origin_y = previous_origin_y;
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn char_at(&self, x: u16, y: u16) -> Option<char> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(self.cells[self.index(x, y)].ch)
    }

    pub fn style_at(&self, x: u16, y: u16) -> Option<Style> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(self.cells[self.index(x, y)].style)
    }

    pub(crate) fn cells(&self) -> &[Cell] {
        &self.cells
    }

    pub(crate) fn sync_from(&mut self, other: &Frame) {
        self.width = other.width;
        self.height = other.height;
        self.clip = Rect::new(0, 0, other.width, other.height);
        self.origin_x = 0;
        self.origin_y = 0;

        if self.cells.len() != other.cells.len() {
            self.cells = vec![Cell::default(); other.cells.len()];
        }

        self.cells.clone_from_slice(other.cells());
    }

    fn index(&self, x: u16, y: u16) -> usize {
        y as usize * self.width as usize + x as usize
    }
}

fn intersect_rects(a: Rect, b: Rect) -> Rect {
    let a_left = a.x as u32;
    let a_top = a.y as u32;
    let a_right = a_left + a.width as u32;
    let a_bottom = a_top + a.height as u32;

    let b_left = b.x as u32;
    let b_top = b.y as u32;
    let b_right = b_left + b.width as u32;
    let b_bottom = b_top + b.height as u32;

    let left = a_left.max(b_left);
    let top = a_top.max(b_top);
    let right = a_right.min(b_right);
    let bottom = a_bottom.min(b_bottom);

    if right <= left || bottom <= top {
        return Rect::new(left as u16, top as u16, 0, 0);
    }

    Rect::new(
        left as u16,
        top as u16,
        (right - left) as u16,
        (bottom - top) as u16,
    )
}

#[cfg(test)]
mod tests {
    use super::Frame;
    use crate::{Color, Modifier, Rect, Style};

    #[test]
    fn print_stops_at_frame_right_edge() {
        let mut frame = Frame::new(5, 1);
        frame.print(3, 0, "abcd");

        assert_eq!(frame.char_at(0, 0), Some(' '));
        assert_eq!(frame.char_at(1, 0), Some(' '));
        assert_eq!(frame.char_at(2, 0), Some(' '));
        assert_eq!(frame.char_at(3, 0), Some('a'));
        assert_eq!(frame.char_at(4, 0), Some('b'));
    }

    #[test]
    fn print_out_of_bounds_is_ignored() {
        let mut frame = Frame::new(4, 2);
        frame.print(20, 0, "x");
        frame.print(0, 20, "y");

        for y in 0..frame.height() {
            for x in 0..frame.width() {
                assert_eq!(frame.char_at(x, y), Some(' '));
            }
        }
    }

    #[test]
    fn render_in_clips_to_area() {
        let mut frame = Frame::new(8, 3);
        frame.render_in(Rect::new(2, 1, 3, 1), |f| {
            f.print(0, 0, "hello");
            f.print(0, 1, "ignored");
        });

        assert_eq!(frame.char_at(2, 1), Some('h'));
        assert_eq!(frame.char_at(3, 1), Some('e'));
        assert_eq!(frame.char_at(4, 1), Some('l'));
        assert_eq!(frame.char_at(5, 1), Some(' '));
        assert_eq!(frame.char_at(2, 2), Some(' '));
    }

    #[test]
    fn render_in_nested_areas_intersect_correctly() {
        let mut frame = Frame::new(8, 4);

        frame.render_in(Rect::new(1, 1, 5, 2), |f| {
            f.render_in(Rect::new(2, 0, 2, 1), |f| {
                f.print(0, 0, "abcd");
            });
        });

        assert_eq!(frame.char_at(3, 1), Some('a'));
        assert_eq!(frame.char_at(4, 1), Some('b'));
        assert_eq!(frame.char_at(5, 1), Some(' '));
    }

    #[test]
    fn render_in_restores_origin_after_closure() {
        let mut frame = Frame::new(6, 2);

        frame.render_in(Rect::new(3, 0, 2, 1), |f| {
            f.print(0, 0, "x");
        });

        frame.print(0, 0, "y");

        assert_eq!(frame.char_at(3, 0), Some('x'));
        assert_eq!(frame.char_at(0, 0), Some('y'));
    }

    #[test]
    fn print_styled_writes_style_for_visible_cells() {
        let mut frame = Frame::new(6, 2);
        let style = Style::new()
            .fg(Color::Ansi(33))
            .bg(Color::Rgb(10, 20, 30))
            .modifier(Modifier::Bold);

        frame.print_styled(1, 0, "abc", style);

        assert_eq!(frame.style_at(1, 0), Some(style));
        assert_eq!(frame.style_at(2, 0), Some(style));
        assert_eq!(frame.style_at(3, 0), Some(style));
        assert_eq!(frame.style_at(0, 0), Some(Style::default()));
    }

    #[test]
    fn print_styled_clips_without_touching_outside_style() {
        let mut frame = Frame::new(4, 1);
        let style = Style::new().fg(Color::Ansi(2));

        frame.print_styled(3, 0, "zz", style);

        assert_eq!(frame.char_at(3, 0), Some('z'));
        assert_eq!(frame.style_at(3, 0), Some(style));
        assert_eq!(frame.style_at(2, 0), Some(Style::default()));
    }
}
