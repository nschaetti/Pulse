use std::{
    io::{self, stdout},
    time::Duration,
};

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyEvent},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::{backend::TerminalBackend, App, Command, Frame};

pub fn run<A, F>(app: &mut A, mut map_key: F) -> io::Result<()>
where
    A: App,
    F: FnMut(KeyEvent) -> Option<A::Msg>,
{
    let _terminal_guard = TerminalGuard::enter()?;

    let (width, height) = terminal::size()?;
    let mut frame = Frame::new(width, height);
    let mut backend = TerminalBackend::new(width, height);

    app.init();
    draw(app, &mut frame, &mut backend)?;

    loop {
        if !event::poll(Duration::from_millis(250))? {
            continue;
        }

        match event::read()? {
            Event::Key(key) => {
                let Some(msg) = map_key(key) else {
                    continue;
                };

                if process_message(app, msg) {
                    break;
                }

                draw(app, &mut frame, &mut backend)?;
            }
            Event::Resize(width, height) => {
                frame = Frame::new(width, height);
                draw(app, &mut frame, &mut backend)?;
            }
            _ => {}
        }
    }

    Ok(())
}

fn draw<A: App>(app: &A, frame: &mut Frame, backend: &mut TerminalBackend) -> io::Result<()> {
    frame.clear();
    app.view(frame);
    backend.render(frame)
}

fn process_message<A: App>(app: &mut A, msg: A::Msg) -> bool {
    let mut pending = Some(msg);

    while let Some(next_msg) = pending.take() {
        match app.update(next_msg) {
            Command::None => {}
            Command::Quit => return true,
            Command::Emit(msg) => pending = Some(msg),
        }
    }

    false
}

struct TerminalGuard;

impl TerminalGuard {
    fn enter() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen, Hide)?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = execute!(stdout(), Show, LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
    }
}
