use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use pulse::{run, update_child, App, Command, Component, Frame, Rect};

struct Sidebar {
    selected: usize,
}

#[derive(Clone, Copy)]
enum SidebarMsg {
    Up,
    Down,
}

impl Component for Sidebar {
    type Msg = SidebarMsg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            SidebarMsg::Up => {
                self.selected = self.selected.saturating_sub(1);
                Command::none()
            }
            SidebarMsg::Down => {
                self.selected = (self.selected + 1).min(2);
                Command::none()
            }
        }
    }

    fn view(&self, frame: &mut Frame, area: Rect) {
        frame.render_in(area, |frame| {
            frame.print(0, 0, "Sidebar");

            let items = ["Home", "Metrics", "Settings"];
            for (idx, item) in items.iter().enumerate() {
                let marker = if idx == self.selected { ">" } else { " " };
                frame.print(0, (idx + 2) as u16, &format!("{} {}", marker, item));
            }
        });
    }
}

struct Content {
    value: i32,
}

#[derive(Clone, Copy)]
enum ContentMsg {
    Increment,
    Decrement,
}

impl Component for Content {
    type Msg = ContentMsg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            ContentMsg::Increment => {
                self.value += 1;
                Command::none()
            }
            ContentMsg::Decrement => {
                self.value -= 1;
                Command::none()
            }
        }
    }

    fn view(&self, frame: &mut Frame, area: Rect) {
        frame.render_in(area, |frame| {
            frame.print(0, 0, "Content");
            frame.print(0, 2, &format!("Counter: {}", self.value));
            frame.print(0, 4, "+ / - to change value");
        });
    }
}

struct Dashboard {
    sidebar: Sidebar,
    content: Content,
}

enum Msg {
    Sidebar(SidebarMsg),
    Content(ContentMsg),
    Quit,
}

impl App for Dashboard {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            Msg::Sidebar(msg) => update_child(&mut self.sidebar, msg, Msg::Sidebar),
            Msg::Content(msg) => update_child(&mut self.content, msg, Msg::Content),
            Msg::Quit => Command::quit(),
        }
    }

    fn view(&self, frame: &mut Frame) {
        let root = Rect::new(0, 0, frame.width(), frame.height());
        let (sidebar_area, content_area) = root.split_horizontal(root.width / 3);

        self.sidebar.view(frame, sidebar_area);
        self.content.view(frame, content_area);

        frame.print(
            0,
            frame.height().saturating_sub(1),
            "Arrows: navigate | +/-: value | q: quit",
        );
    }
}

fn map_key(key: KeyEvent) -> Option<Msg> {
    if key.kind != KeyEventKind::Press {
        return None;
    }

    match key.code {
        KeyCode::Up => Some(Msg::Sidebar(SidebarMsg::Up)),
        KeyCode::Down => Some(Msg::Sidebar(SidebarMsg::Down)),
        KeyCode::Char('+') => Some(Msg::Content(ContentMsg::Increment)),
        KeyCode::Char('-') => Some(Msg::Content(ContentMsg::Decrement)),
        KeyCode::Char('q') => Some(Msg::Quit),
        _ => None,
    }
}

fn main() -> std::io::Result<()> {
    let mut app = Dashboard {
        sidebar: Sidebar { selected: 0 },
        content: Content { value: 0 },
    };
    run(&mut app, map_key)
}
