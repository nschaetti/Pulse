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

#[cfg(test)]
mod tests {
    use super::Rect;

    #[test]
    fn split_vertical_clamps_to_height() {
        let root = Rect::new(2, 3, 10, 4);
        let (top, bottom) = root.split_vertical(999);

        assert_eq!(top, Rect::new(2, 3, 10, 4));
        assert_eq!(bottom, Rect::new(2, 7, 10, 0));
    }

    #[test]
    fn split_vertical_handles_zero() {
        let root = Rect::new(0, 0, 8, 5);
        let (top, bottom) = root.split_vertical(0);

        assert_eq!(top, Rect::new(0, 0, 8, 0));
        assert_eq!(bottom, Rect::new(0, 0, 8, 5));
    }

    #[test]
    fn split_horizontal_clamps_to_width() {
        let root = Rect::new(1, 4, 6, 3);
        let (left, right) = root.split_horizontal(100);

        assert_eq!(left, Rect::new(1, 4, 6, 3));
        assert_eq!(right, Rect::new(7, 4, 0, 3));
    }

    #[test]
    fn split_horizontal_preserves_total_width() {
        let root = Rect::new(0, 1, 9, 2);
        let (left, right) = root.split_horizontal(4);

        assert_eq!(left, Rect::new(0, 1, 4, 2));
        assert_eq!(right, Rect::new(4, 1, 5, 2));
        assert_eq!(left.width + right.width, root.width);
    }
}
