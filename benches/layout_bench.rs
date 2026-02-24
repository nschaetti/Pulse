use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pulse::{Constraint, Direction, LayoutNode, Padding, Rect, Slot};

fn dashboard_layout() -> LayoutNode {
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
                            Constraint::Percent(25),
                            LayoutNode::leaf("sidebar").with_padding(Padding::all(1)),
                        ),
                        Slot::new(
                            Constraint::Fill,
                            LayoutNode::split(
                                "content_stack",
                                Direction::Vertical,
                                [
                                    Slot::new(Constraint::Percent(70), LayoutNode::leaf("main")),
                                    Slot::new(Constraint::Fill, LayoutNode::leaf("log")),
                                ],
                            )
                            .with_padding(Padding::all(1)),
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

fn bench_layout_resolve_desktop(c: &mut Criterion) {
    c.bench_function("layout/resolve_dashboard_120x40", |b| {
        let layout = dashboard_layout();
        let rect = Rect::new(0, 0, 120, 40);

        b.iter(|| {
            let resolved = layout.resolve(rect);
            black_box(resolved.area("main"));
        });
    });
}

fn bench_layout_resolve_small_terminal(c: &mut Criterion) {
    c.bench_function("layout/resolve_dashboard_40x12", |b| {
        let layout = dashboard_layout();
        let rect = Rect::new(0, 0, 40, 12);

        b.iter(|| {
            let resolved = layout.resolve(rect);
            black_box(resolved.area("sidebar"));
        });
    });
}

fn bench_layout_percent_heavy(c: &mut Criterion) {
    c.bench_function("layout/percent_heavy_200x60", |b| {
        let layout = LayoutNode::split(
            "root",
            Direction::Horizontal,
            [
                Slot::new(Constraint::Percent(10), LayoutNode::leaf("a")),
                Slot::new(Constraint::Percent(20), LayoutNode::leaf("b")),
                Slot::new(Constraint::Percent(30), LayoutNode::leaf("c")),
                Slot::new(Constraint::Percent(40), LayoutNode::leaf("d")),
                Slot::new(Constraint::Fill, LayoutNode::leaf("tail")),
            ],
        );

        b.iter(|| {
            let resolved = layout.resolve(Rect::new(0, 0, 200, 60));
            black_box(resolved.zones().len());
        });
    });
}

criterion_group!(
    layout_benches,
    bench_layout_resolve_desktop,
    bench_layout_resolve_small_terminal,
    bench_layout_percent_heavy
);
criterion_main!(layout_benches);
