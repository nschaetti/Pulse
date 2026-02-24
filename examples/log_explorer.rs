use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use pulse::{
    run, App, Block, Command, Constraint, Direction, Frame, LayoutNode, List, Padding, Rect, Slot,
    Text,
};

const SOURCES: [&str; 9] = [
    "api",
    "worker",
    "scheduler",
    "gateway",
    "auth",
    "billing",
    "notifications",
    "search",
    "storage",
];

struct LogExplorer {
    layout: LayoutNode,
    selected_source: usize,
}

enum Msg {
    Up,
    Down,
    Quit,
}

impl App for LogExplorer {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            Msg::Up => {
                self.selected_source = self.selected_source.saturating_sub(1);
                Command::none()
            }
            Msg::Down => {
                self.selected_source =
                    (self.selected_source + 1).min(SOURCES.len().saturating_sub(1));
                Command::none()
            }
            Msg::Quit => Command::quit(),
        }
    }

    fn view(&self, frame: &mut Frame) {
        let root = Rect::new(0, 0, frame.width(), frame.height());
        let zones = self.layout.resolve(root);

        if let Some(area) = zones.area("filters") {
            let block = Block::new().title("Filters");
            block.render(frame, area);
            Text::new("level=info  env=prod  since=15m").render(frame, block.inner_area(area));
        }

        if let Some(area) = zones.area("sources") {
            let block = Block::new().title("Sources");
            block.render(frame, area);
            List::new(SOURCES)
                .selected(self.selected_source)
                .render(frame, block.inner_area(area));
        }

        if let Some(area) = zones.area("logs") {
            let block = Block::new().title("Logs");
            block.render(frame, area);

            let source = SOURCES[self.selected_source];
            let lines = [
                format!("12:04:13 {} INFO  request completed in 14ms", source),
                format!("12:04:12 {} INFO  accepted connection", source),
                format!(
                    "12:04:11 {} WARN  retrying transient upstream error",
                    source
                ),
                format!("12:04:10 {} INFO  cache warmed", source),
                format!("12:04:09 {} INFO  background task heartbeat", source),
            ];
            Text::new(lines.join("\n")).render(frame, block.inner_area(area));
        }

        if let Some(area) = zones.area("status") {
            Text::new("up/down or j/k: source | q: quit").render(frame, area);
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
                LayoutNode::leaf("filters").with_padding(Padding::symmetric(1, 2)),
            ),
            Slot::new(
                Constraint::Fill,
                LayoutNode::split(
                    "body",
                    Direction::Horizontal,
                    [
                        Slot::new(
                            Constraint::Percent(30),
                            LayoutNode::leaf("sources").with_padding(Padding::all(1)),
                        ),
                        Slot::new(
                            Constraint::Fill,
                            LayoutNode::leaf("logs").with_padding(Padding::all(1)),
                        ),
                    ],
                ),
            ),
            Slot::new(
                Constraint::Fixed(1),
                LayoutNode::leaf("status").with_padding(Padding::symmetric(0, 2)),
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
    let mut app = LogExplorer {
        layout: build_layout(),
        selected_source: 0,
    };
    run(&mut app, map_key)
}
