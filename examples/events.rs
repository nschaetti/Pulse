use std::time::Duration;

use crossterm::event::{KeyCode, KeyEventKind};
use pulse::{run_with_events, App, Command, Event, Frame};

struct EventApp {
    ticks: u64,
    width: u16,
    height: u16,
}

enum Msg {
    Tick,
    Resize(u16, u16),
    Quit,
}

impl App for EventApp {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            Msg::Tick => {
                self.ticks = self.ticks.saturating_add(1);
                Command::none()
            }
            Msg::Resize(width, height) => {
                self.width = width;
                self.height = height;
                Command::none()
            }
            Msg::Quit => Command::quit(),
        }
    }

    fn view(&self, frame: &mut Frame) {
        frame.print(0, 0, "Event Runtime Demo");
        frame.print(0, 2, &format!("Ticks: {}", self.ticks));
        frame.print(
            0,
            3,
            &format!("Last resize: {} x {}", self.width, self.height),
        );
        frame.print(0, 5, "Press q to quit");
    }
}

fn map_event(event: Event) -> Option<Msg> {
    match event {
        Event::Tick => Some(Msg::Tick),
        Event::Resize { width, height } => Some(Msg::Resize(width, height)),
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => Some(Msg::Quit),
            _ => None,
        },
        _ => None,
    }
}

fn main() -> std::io::Result<()> {
    let mut app = EventApp {
        ticks: 0,
        width: 0,
        height: 0,
    };

    run_with_events(&mut app, Duration::from_millis(250), map_event)
}
