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

#[cfg(test)]
mod tests {
    use crate::{App, Command, Frame};

    use super::process_message;

    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    enum Msg {
        Start,
        StepA,
        StepB,
        Quit,
    }

    struct TestApp {
        updates: Vec<Msg>,
    }

    impl TestApp {
        fn new() -> Self {
            Self {
                updates: Vec::new(),
            }
        }
    }

    impl App for TestApp {
        type Msg = Msg;

        fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
            self.updates.push(msg);

            match msg {
                Msg::Start => Command::Emit(Msg::StepA),
                Msg::StepA => Command::Emit(Msg::StepB),
                Msg::StepB => Command::None,
                Msg::Quit => Command::Quit,
            }
        }

        fn view(&self, _frame: &mut Frame) {}
    }

    #[test]
    fn process_message_runs_emit_chain_in_order() {
        let mut app = TestApp::new();

        let should_quit = process_message(&mut app, Msg::Start);

        assert!(!should_quit);
        assert_eq!(app.updates, vec![Msg::Start, Msg::StepA, Msg::StepB]);
    }

    #[test]
    fn process_message_returns_true_on_quit() {
        let mut app = TestApp::new();

        let should_quit = process_message(&mut app, Msg::Quit);

        assert!(should_quit);
        assert_eq!(app.updates, vec![Msg::Quit]);
    }
}
