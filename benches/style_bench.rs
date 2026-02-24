use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pulse::{
    apply_input_edit, Color, Frame, Input, InputEdit, Paragraph, Rect, StatusBar, Style, WrapMode,
};

fn bench_style_full_repaint(c: &mut Criterion) {
    c.bench_function("style/full_repaint_120x40", |b| {
        let mut frame = Frame::new(120, 40);
        let line = "x".repeat(120);
        let style_a = Style::new().fg(Color::Ansi(45)).bg(Color::Ansi(17));
        let style_b = Style::new().fg(Color::Ansi(230)).bg(Color::Ansi(52));

        b.iter(|| {
            frame.clear();
            for y in 0..frame.height() {
                let style = if y % 2 == 0 { style_a } else { style_b };
                frame.print_styled(0, y, &line, style);
            }
            black_box(frame.style_at(0, 0));
        });
    });
}

fn bench_style_only_diff(c: &mut Criterion) {
    c.bench_function("style/style_only_diff_120x40", |b| {
        let mut frame = Frame::new(120, 40);
        let line = " ".repeat(120);
        let style_a = Style::new().bg(Color::Ansi(18));
        let style_b = Style::new().bg(Color::Ansi(19));

        b.iter(|| {
            for y in 0..frame.height() {
                frame.print_styled(0, y, &line, style_a);
            }
            for y in 0..frame.height() {
                frame.print_styled(0, y, &line, style_b);
            }
            black_box(frame.style_at(50, 20));
        });
    });
}

fn bench_style_mixed_widgets_frame(c: &mut Criterion) {
    c.bench_function("style/mixed_widgets_frame", |b| {
        let mut frame = Frame::new(120, 40);
        let paragraph = Paragraph::new(
            "Pulse benchmarks wrapped text, themed status bars, and editable inputs to validate \
             style-heavy rendering paths.",
        )
        .style(Style::new().fg(Color::Ansi(252)).bg(Color::Ansi(17)))
        .wrap(WrapMode::Word);
        let status = StatusBar::new()
            .left("pulse bench")
            .right("report-only")
            .style(Style::new().bg(Color::Ansi(236)))
            .left_style(Style::new().fg(Color::Ansi(154)))
            .right_style(Style::new().fg(Color::Ansi(229)));
        let input = Input::new()
            .value("search:logs")
            .cursor(6)
            .focused(true)
            .style(Style::new().fg(Color::Ansi(252)).bg(Color::Ansi(22)))
            .cursor_style(Style::new().fg(Color::Ansi(16)).bg(Color::Ansi(39)));

        b.iter(|| {
            frame.clear();
            paragraph.render(&mut frame, Rect::new(1, 1, 70, 4));
            input.render(&mut frame, Rect::new(1, 7, 40, 1));
            status.render(&mut frame, Rect::new(0, 39, 120, 1));
            black_box(frame.char_at(1, 1));
        });
    });
}

fn bench_style_input_edit_cycle(c: &mut Criterion) {
    c.bench_function("style/input_edit_cycle", |b| {
        b.iter(|| {
            let mut value = String::from("pulse");
            let mut cursor = value.chars().count();

            apply_input_edit(&mut value, &mut cursor, InputEdit::Insert('-'));
            apply_input_edit(&mut value, &mut cursor, InputEdit::Insert('x'));
            apply_input_edit(&mut value, &mut cursor, InputEdit::Left);
            apply_input_edit(&mut value, &mut cursor, InputEdit::Backspace);
            apply_input_edit(&mut value, &mut cursor, InputEdit::End);

            black_box((value, cursor));
        });
    });
}

criterion_group!(
    style_benches,
    bench_style_full_repaint,
    bench_style_only_diff,
    bench_style_mixed_widgets_frame,
    bench_style_input_edit_cycle
);
criterion_main!(style_benches);
