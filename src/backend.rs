use std::io::{self, stdout, Stdout, Write};

use crossterm::{
    cursor::MoveTo,
    queue,
    style::{
        Attribute, Color as CrosstermColor, Print, ResetColor, SetAttribute, SetBackgroundColor,
        SetForegroundColor,
    },
    terminal::{Clear, ClearType},
};

use crate::{Color, Frame, Modifier, Style};

pub struct TerminalBackend {
    stdout: Stdout,
    previous: Frame,
    active_style: Style,
}

impl TerminalBackend {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            stdout: stdout(),
            previous: Frame::new(width, height),
            active_style: Style::default(),
        }
    }

    pub fn render(&mut self, current: &Frame) -> io::Result<()> {
        if self.previous.width() != current.width() || self.previous.height() != current.height() {
            queue!(self.stdout, Clear(ClearType::All))?;
            self.previous = Frame::new(current.width(), current.height());
            self.active_style = Style::default();
        }

        let width = current.width() as usize;
        for idx in 0..current.cells().len() {
            let prev_cell = self.previous.cells()[idx];
            let curr_cell = current.cells()[idx];
            if prev_cell == curr_cell {
                continue;
            }

            let x = (idx % width) as u16;
            let y = (idx / width) as u16;
            self.apply_style(curr_cell.style)?;
            queue!(self.stdout, MoveTo(x, y), Print(curr_cell.ch))?;
        }

        queue!(self.stdout, ResetColor, SetAttribute(Attribute::Reset))?;
        self.active_style = Style::default();
        self.stdout.flush()?;
        self.previous.sync_from(current);
        Ok(())
    }

    fn apply_style(&mut self, style: Style) -> io::Result<()> {
        if self.active_style == style {
            return Ok(());
        }

        queue!(self.stdout, SetAttribute(Attribute::Reset))?;
        queue!(
            self.stdout,
            SetForegroundColor(map_color(style.fg.unwrap_or(Color::Default))),
            SetBackgroundColor(map_color(style.bg.unwrap_or(Color::Default)))
        )?;

        if style.modifiers.contains(Modifier::Bold) {
            queue!(self.stdout, SetAttribute(Attribute::Bold))?;
        }
        if style.modifiers.contains(Modifier::Dim) {
            queue!(self.stdout, SetAttribute(Attribute::Dim))?;
        }
        if style.modifiers.contains(Modifier::Italic) {
            queue!(self.stdout, SetAttribute(Attribute::Italic))?;
        }
        if style.modifiers.contains(Modifier::Underline) {
            queue!(self.stdout, SetAttribute(Attribute::Underlined))?;
        }
        if style.modifiers.contains(Modifier::Reverse) {
            queue!(self.stdout, SetAttribute(Attribute::Reverse))?;
        }

        self.active_style = style;
        Ok(())
    }
}

fn map_color(color: Color) -> CrosstermColor {
    match color {
        Color::Default => CrosstermColor::Reset,
        Color::Ansi(value) => CrosstermColor::AnsiValue(value),
        Color::Rgb(r, g, b) => CrosstermColor::Rgb { r, g, b },
    }
}

#[cfg(test)]
mod tests {
    use super::{map_color, CrosstermColor};
    use crate::Color;

    #[test]
    fn map_color_supports_ansi_and_rgb() {
        assert_eq!(map_color(Color::Default), CrosstermColor::Reset);
        assert_eq!(map_color(Color::Ansi(42)), CrosstermColor::AnsiValue(42));
        assert_eq!(
            map_color(Color::Rgb(1, 2, 3)),
            CrosstermColor::Rgb { r: 1, g: 2, b: 3 }
        );
    }
}
