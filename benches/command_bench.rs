use std::collections::VecDeque;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pulse::Command;

fn drain_queue(start: Command<u32>) -> usize {
    let mut processed = 0_usize;
    let mut queue = VecDeque::new();
    schedule(start, &mut queue);

    while let Some(msg) = queue.pop_front() {
        processed = processed.saturating_add(1);

        let cmd = if msg % 7 == 0 {
            Command::batch([Command::emit(msg + 1), Command::emit(msg + 2)])
        } else if msg >= 10_000 {
            Command::none()
        } else {
            Command::emit(msg + 1)
        };

        schedule(cmd, &mut queue);
        if processed > 50_000 {
            break;
        }
    }

    processed
}

fn schedule(command: Command<u32>, queue: &mut VecDeque<u32>) {
    match command {
        Command::None => {}
        Command::Quit => {}
        Command::Emit(msg) => {
            queue.push_back(msg);
        }
        Command::Batch(commands) => {
            for command in commands {
                schedule(command, queue);
            }
        }
    }
}

fn bench_command_emit_chain(c: &mut Criterion) {
    c.bench_function("command/emit_chain", |b| {
        b.iter(|| {
            let processed = drain_queue(Command::emit(1));
            black_box(processed);
        });
    });
}

fn bench_command_nested_batch(c: &mut Criterion) {
    c.bench_function("command/nested_batch", |b| {
        b.iter(|| {
            let root = Command::batch([
                Command::emit(1),
                Command::batch([
                    Command::emit(2),
                    Command::batch([Command::emit(3), Command::emit(4), Command::emit(5)]),
                ]),
                Command::emit(6),
            ]);
            let processed = drain_queue(root);
            black_box(processed);
        });
    });
}

fn bench_command_map_on_nested_batch(c: &mut Criterion) {
    c.bench_function("command/map_nested_batch", |b| {
        b.iter(|| {
            let root = Command::batch([
                Command::emit(1_u32),
                Command::batch([Command::emit(2_u32), Command::emit(3_u32)]),
                Command::batch([
                    Command::emit(4_u32),
                    Command::batch([Command::emit(5_u32), Command::quit()]),
                ]),
            ]);
            let mapped = root.map(|v| v + 10);
            black_box(mapped);
        });
    });
}

criterion_group!(
    command_benches,
    bench_command_emit_chain,
    bench_command_nested_batch,
    bench_command_map_on_nested_batch
);
criterion_main!(command_benches);
