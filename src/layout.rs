#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn split_vertical(&self, top_height: u16) -> (Rect, Rect) {
        let top_h = top_height.min(self.height);
        let bottom_h = self.height.saturating_sub(top_h);

        (
            Rect::new(self.x, self.y, self.width, top_h),
            Rect::new(self.x, self.y.saturating_add(top_h), self.width, bottom_h),
        )
    }

    pub fn split_horizontal(&self, left_width: u16) -> (Rect, Rect) {
        let left_w = left_width.min(self.width);
        let right_w = self.width.saturating_sub(left_w);

        (
            Rect::new(self.x, self.y, left_w, self.height),
            Rect::new(self.x.saturating_add(left_w), self.y, right_w, self.height),
        )
    }
}
