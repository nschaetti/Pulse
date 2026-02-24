use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use pulse::{
    run, App, Command, Constraint, Direction, Frame, LayoutNode, Padding, Rect, Slot, Text,
};

struct InterfaceDemo {
    layout: LayoutNode,
}

enum Msg {
    Quit,
}

impl App for InterfaceDemo {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            Msg::Quit => Command::quit(),
        }
    }

    fn view(&self, frame: &mut Frame) {
        let root = Rect::new(0, 0, frame.width(), frame.height());
        let resolved = self.layout.resolve(root);

        if let Some(area) = resolved.area("header") {
            Text::new("Pulse Interface Layout").render(frame, area);
        }

        if let Some(area) = resolved.area("sidebar") {
            Text::new("Navigation\n- Overview\n- Metrics\n- Settings").render(frame, area);
        }

        if let Some(area) = resolved.area("content") {
            Text::new("Content Area\n\nThis zone is ready for future widgets.").render(frame, area);
        }

        if let Some(area) = resolved.area("footer") {
            Text::new("q: quit").render(frame, area);
        }
    }
}

fn build_layout() -> LayoutNode {
    LayoutNode::split(
        "root",
        Direction::Vertical,
        [
            Slot::new(
                Constraint::Fixed(3),
                LayoutNode::leaf("header").with_padding(Padding::symmetric(1, 2)),
            ),
            Slot::new(
                Constraint::Fill,
                LayoutNode::split(
                    "body",
                    Direction::Horizontal,
                    [
                        Slot::new(
                            Constraint::Percent(30),
                            LayoutNode::leaf("sidebar").with_padding(Padding::all(1)),
                        ),
                        Slot::new(
                            Constraint::Fill,
                            LayoutNode::leaf("content").with_padding(Padding::all(1)),
                        ),
                    ],
                ),
            ),
            Slot::new(
                Constraint::Fixed(2),
                LayoutNode::leaf("footer").with_padding(Padding::symmetric(0, 2)),
            ),
        ],
    )
}

fn map_key(key: KeyEvent) -> Option<Msg> {
    if key.kind != KeyEventKind::Press {
        return None;
    }

    match key.code {
        KeyCode::Char('q') => Some(Msg::Quit),
        _ => None,
    }
}

fn main() -> std::io::Result<()> {
    let mut app = InterfaceDemo {
        layout: build_layout(),
    };

    run(&mut app, map_key)
}
