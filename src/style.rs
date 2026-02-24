#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Color {
    Default,
    Ansi(u8),
    Rgb(u8, u8, u8),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Modifier {
    Bold,
    Dim,
    Italic,
    Underline,
    Reverse,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ModifierSet {
    bits: u16,
}

impl ModifierSet {
    pub fn insert(mut self, modifier: Modifier) -> Self {
        self.bits |= modifier.bit();
        self
    }

    pub fn remove(mut self, modifier: Modifier) -> Self {
        self.bits &= !modifier.bit();
        self
    }

    pub fn contains(self, modifier: Modifier) -> bool {
        self.bits & modifier.bit() != 0
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub modifiers: ModifierSet,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    pub fn modifier(mut self, modifier: Modifier) -> Self {
        self.modifiers = self.modifiers.insert(modifier);
        self
    }

    pub fn remove(mut self, modifier: Modifier) -> Self {
        self.modifiers = self.modifiers.remove(modifier);
        self
    }
}

impl Modifier {
    fn bit(self) -> u16 {
        match self {
            Modifier::Bold => 1 << 0,
            Modifier::Dim => 1 << 1,
            Modifier::Italic => 1 << 2,
            Modifier::Underline => 1 << 3,
            Modifier::Reverse => 1 << 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Color, Modifier, Style};

    #[test]
    fn style_builder_sets_colors_and_modifiers() {
        let style = Style::new()
            .fg(Color::Ansi(12))
            .bg(Color::Rgb(10, 20, 30))
            .modifier(Modifier::Bold)
            .modifier(Modifier::Underline);

        assert_eq!(style.fg, Some(Color::Ansi(12)));
        assert_eq!(style.bg, Some(Color::Rgb(10, 20, 30)));
        assert!(style.modifiers.contains(Modifier::Bold));
        assert!(style.modifiers.contains(Modifier::Underline));
    }

    #[test]
    fn remove_modifier_clears_flag_only() {
        let style = Style::new()
            .modifier(Modifier::Bold)
            .modifier(Modifier::Italic)
            .remove(Modifier::Bold);

        assert!(!style.modifiers.contains(Modifier::Bold));
        assert!(style.modifiers.contains(Modifier::Italic));
    }
}
