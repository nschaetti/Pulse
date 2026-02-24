use std::{
    collections::VecDeque,
    io::{self, stdout},
    time::Duration,
};

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event as CrosstermEvent, KeyEvent},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::{backend::TerminalBackend, App, Command, Event, Frame};

pub fn run<A, F>(app: &mut A, mut map_key: F) -> io::Result<()>
where
    A: App,
    F: FnMut(KeyEvent) -> Option<A::Msg>,
{
    run_with_events(app, Duration::from_millis(250), move |event| match event {
        Event::Key(key) => map_key(key),
        Event::Resize { .. } | Event::Tick => None,
    })
}

pub fn run_with_events<A, F>(app: &mut A, tick_rate: Duration, mut map_event: F) -> io::Result<()>
where
    A: App,
    F: FnMut(Event) -> Option<A::Msg>,
{
    let _terminal_guard = TerminalGuard::enter()?;

    let (width, height) = terminal::size()?;
    let mut frame = Frame::new(width, height);
    let mut backend = TerminalBackend::new(width, height);

    app.init();
    draw(app, &mut frame, &mut backend)?;

    loop {
        if !event::poll(tick_rate)? {
            if process_event(app, Event::Tick, &mut map_event) {
                break;
            }
            continue;
        }

        match event::read()? {
            CrosstermEvent::Key(key) => {
                if process_event(app, Event::Key(key), &mut map_event) {
                    break;
                }

                draw(app, &mut frame, &mut backend)?;
            }
            CrosstermEvent::Resize(width, height) => {
                frame = Frame::new(width, height);

                if process_event(app, Event::Resize { width, height }, &mut map_event) {
                    break;
                }

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
    let mut pending = VecDeque::from([msg]);

    while let Some(next_msg) = pending.pop_front() {
        if schedule_command(app.update(next_msg), &mut pending) {
            return true;
        }
    }

    false
}

fn schedule_command<Msg>(command: Command<Msg>, pending: &mut VecDeque<Msg>) -> bool {
    match command {
        Command::None => false,
        Command::Quit => true,
        Command::Emit(msg) => {
            pending.push_back(msg);
            false
        }
        Command::Batch(commands) => {
            for command in commands {
                if schedule_command(command, pending) {
                    return true;
                }
            }
            false
        }
    }
}

fn process_event<A, F>(app: &mut A, event: Event, map_event: &mut F) -> bool
where
    A: App,
    F: FnMut(Event) -> Option<A::Msg>,
{
    match map_event(event) {
        Some(msg) => process_message(app, msg),
        None => false,
    }
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
    use crate::{App, Command, Event, Frame};

    use super::{process_event, process_message};

    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    enum Msg {
        Start,
        StepA,
        StepB,
        BatchStart,
        BatchNested,
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
                Msg::BatchStart => Command::Batch(vec![
                    Command::Emit(Msg::StepA),
                    Command::Emit(Msg::StepB),
                    Command::Emit(Msg::BatchNested),
                ]),
                Msg::BatchNested => {
                    Command::Batch(vec![Command::Emit(Msg::StepA), Command::Emit(Msg::Quit)])
                }
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

    #[test]
    fn process_event_ignores_unmapped_events() {
        let mut app = TestApp::new();
        let mut mapper = |_event| None;

        let should_quit = process_event(&mut app, Event::Tick, &mut mapper);

        assert!(!should_quit);
        assert!(app.updates.is_empty());
    }

    #[test]
    fn process_event_maps_event_to_message() {
        let mut app = TestApp::new();
        let mut mapper = |event| match event {
            Event::Tick => Some(Msg::Start),
            _ => None,
        };

        let should_quit = process_event(&mut app, Event::Tick, &mut mapper);

        assert!(!should_quit);
        assert_eq!(app.updates, vec![Msg::Start, Msg::StepA, Msg::StepB]);
    }

    #[test]
    fn process_message_runs_batch_with_fifo_order() {
        let mut app = TestApp::new();

        let should_quit = process_message(&mut app, Msg::BatchStart);

        assert!(should_quit);
        assert_eq!(
            app.updates,
            vec![
                Msg::BatchStart,
                Msg::StepA,
                Msg::StepB,
                Msg::BatchNested,
                Msg::StepB,
                Msg::StepA,
                Msg::Quit,
            ]
        );
    }
}
