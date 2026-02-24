use crate::Rect;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cell {
    pub ch: char,
}

impl Default for Cell {
    fn default() -> Self {
        Self { ch: ' ' }
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
        }
    }

    pub fn print(&mut self, x: u16, y: u16, text: &str) {
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
