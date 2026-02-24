use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use pulse::{
    run, App, Block, Command, Constraint, Direction, Frame, LayoutNode, List, Padding, Rect, Slot,
    Text,
};

const NAV_ITEMS: [&str; 7] = [
    "Overview",
    "Metrics",
    "Alerts",
    "Deployments",
    "Logs",
    "Config",
    "About",
];

struct AdminConsole {
    layout: LayoutNode,
    selected: usize,
}

enum Msg {
    Up,
    Down,
    Quit,
}

impl App for AdminConsole {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            Msg::Up => {
                self.selected = self.selected.saturating_sub(1);
                Command::none()
            }
            Msg::Down => {
                self.selected = (self.selected + 1).min(NAV_ITEMS.len().saturating_sub(1));
                Command::none()
            }
            Msg::Quit => Command::quit(),
        }
    }

    fn view(&self, frame: &mut Frame) {
        let root = Rect::new(0, 0, frame.width(), frame.height());
        let zones = self.layout.resolve(root);

        if let Some(area) = zones.area("header") {
            let block = Block::new().title("Pulse Admin");
            block.render(frame, area);
            Text::new("Cluster: prod-eu-west | Status: healthy")
                .render(frame, block.inner_area(area));
        }

        if let Some(area) = zones.area("sidebar") {
            let block = Block::new().title("Navigation");
            block.render(frame, area);
            List::new(NAV_ITEMS)
                .selected(self.selected)
                .render(frame, block.inner_area(area));
        }

        if let Some(area) = zones.area("content") {
            let block = Block::new().title("Panel");
            block.render(frame, area);
            Text::new(format!(
                "Selected: {}\n\nUse this area to mount domain widgets.",
                NAV_ITEMS[self.selected]
            ))
            .render(frame, block.inner_area(area));
        }

        if let Some(area) = zones.area("footer") {
            Text::new("up/down or j/k: navigate | q: quit").render(frame, area);
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
                            Constraint::Percent(28),
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
                Constraint::Fixed(1),
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
        KeyCode::Up | KeyCode::Char('k') => Some(Msg::Up),
        KeyCode::Down | KeyCode::Char('j') => Some(Msg::Down),
        KeyCode::Char('q') => Some(Msg::Quit),
        _ => None,
    }
}

fn main() -> std::io::Result<()> {
    let mut app = AdminConsole {
        layout: build_layout(),
        selected: 0,
    };
    run(&mut app, map_key)
}
