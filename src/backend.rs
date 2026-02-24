use std::io::{self, stdout, Stdout, Write};

use crossterm::{
    cursor::MoveTo,
    queue,
    style::Print,
    terminal::{Clear, ClearType},
};

use crate::Frame;

pub struct TerminalBackend {
    stdout: Stdout,
    previous: Frame,
}

impl TerminalBackend {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            stdout: stdout(),
            previous: Frame::new(width, height),
        }
    }

    pub fn render(&mut self, current: &Frame) -> io::Result<()> {
        if self.previous.width() != current.width() || self.previous.height() != current.height() {
            queue!(self.stdout, Clear(ClearType::All))?;
            self.previous = Frame::new(current.width(), current.height());
        }

        let width = current.width() as usize;
        for (idx, (prev_cell, curr_cell)) in self
            .previous
            .cells()
            .iter()
            .zip(current.cells().iter())
            .enumerate()
        {
            if prev_cell == curr_cell {
                continue;
            }

            let x = (idx % width) as u16;
            let y = (idx / width) as u16;
            queue!(self.stdout, MoveTo(x, y), Print(curr_cell.ch))?;
        }

        self.stdout.flush()?;
        self.previous.sync_from(current);
        Ok(())
    }
}
