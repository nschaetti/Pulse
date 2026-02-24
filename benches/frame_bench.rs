use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pulse::{Frame, Rect};

fn bench_frame_full_redraw(c: &mut Criterion) {
    c.bench_function("frame/full_redraw_120x40", |b| {
        let mut frame = Frame::new(120, 40);
        let line = "x".repeat(120);

        b.iter(|| {
            frame.clear();
            for y in 0..frame.height() {
                frame.print(0, y, &line);
            }
            black_box(frame.char_at(119, 39));
        });
    });
}

fn bench_frame_partial_update(c: &mut Criterion) {
    c.bench_function("frame/partial_update_120x40", |b| {
        let mut frame = Frame::new(120, 40);
        let line = "status: 1234567890";

        b.iter(|| {
            frame.render_in(Rect::new(0, 0, 40, 3), |f| {
                f.print(0, 0, line);
                f.print(0, 1, line);
                f.print(0, 2, line);
            });
            black_box(frame.char_at(0, 0));
        });
    });
}

fn bench_frame_nested_clipping(c: &mut Criterion) {
    c.bench_function("frame/nested_clipping", |b| {
        let mut frame = Frame::new(80, 24);
        let text = "abcdefghijklmnopqrstuvwxyz0123456789";

        b.iter(|| {
            frame.clear();
            frame.render_in(Rect::new(2, 2, 40, 10), |f| {
                f.render_in(Rect::new(5, 3, 20, 3), |f| {
                    f.print(0, 0, text);
                    f.print(0, 1, text);
                    f.print(0, 2, text);
                });
            });
            black_box(frame.char_at(7, 5));
        });
    });
}

criterion_group!(
    frame_benches,
    bench_frame_full_redraw,
    bench_frame_partial_update,
    bench_frame_nested_clipping
);
criterion_main!(frame_benches);
