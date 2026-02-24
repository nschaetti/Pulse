use crossterm::event::KeyEvent;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Event {
    Key(KeyEvent),
    Resize { width: u16, height: u16 },
    Tick,
}
