use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use pulse::{
    run, App, Block, Command, Constraint, Direction, Frame, LayoutNode, List, Padding, Rect, Slot,
    Text,
};

const CATEGORIES: [&str; 8] = [
    "General",
    "Appearance",
    "Key Bindings",
    "Network",
    "Notifications",
    "Updates",
    "Advanced",
    "About",
];

struct SettingsApp {
    layout: LayoutNode,
    selected: usize,
}

enum Msg {
    Up,
    Down,
    Quit,
}

impl App for SettingsApp {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            Msg::Up => {
                self.selected = self.selected.saturating_sub(1);
                Command::none()
            }
            Msg::Down => {
                self.selected = (self.selected + 1).min(CATEGORIES.len().saturating_sub(1));
                Command::none()
            }
            Msg::Quit => Command::quit(),
        }
    }

    fn view(&self, frame: &mut Frame) {
        let root = Rect::new(0, 0, frame.width(), frame.height());
        let zones = self.layout.resolve(root);

        if let Some(area) = zones.area("title") {
            let block = Block::new().title("Settings");
            block.render(frame, area);
            Text::new("Pulse configuration center").render(frame, block.inner_area(area));
        }

        if let Some(area) = zones.area("categories") {
            let block = Block::new().title("Categories");
            block.render(frame, area);
            List::new(CATEGORIES)
                .selected(self.selected)
                .render(frame, block.inner_area(area));
        }

        if let Some(area) = zones.area("details") {
            let block = Block::new().title("Details");
            block.render(frame, area);
            let text = format!(
                "Section: {}\n\n- Placeholder option A\n- Placeholder option B\n- Placeholder option C",
                CATEGORIES[self.selected]
            );
            Text::new(text).render(frame, block.inner_area(area));
        }

        if let Some(area) = zones.area("footer") {
            Text::new("up/down or j/k: select section | q: quit").render(frame, area);
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
                LayoutNode::leaf("title").with_padding(Padding::symmetric(1, 2)),
            ),
            Slot::new(
                Constraint::Fill,
                LayoutNode::split(
                    "body",
                    Direction::Horizontal,
                    [
                        Slot::new(
                            Constraint::Percent(35),
                            LayoutNode::leaf("categories").with_padding(Padding::all(1)),
                        ),
                        Slot::new(
                            Constraint::Fill,
                            LayoutNode::leaf("details").with_padding(Padding::all(1)),
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
    let mut app = SettingsApp {
        layout: build_layout(),
        selected: 0,
    };
    run(&mut app, map_key)
}
