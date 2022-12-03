use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day01 part 1", |b| {
        b.iter(|| day01::part1(black_box(&day01::get_input())))
    });

    c.bench_function("day01 part 2", |b| {
        b.iter(|| day01::part2(black_box(&day01::get_input())))
    });

    c.bench_function("day02 part 1", |b| {
        b.iter(|| day02::part1(black_box(&day02::get_input())))
    });

    c.bench_function("day02 part 2", |b| {
        b.iter(|| day02::part2(black_box(&day02::get_input())))
    });

    c.bench_function("day03 part 1", |b| {
        b.iter(|| day03::part1(black_box(&day03::get_input())))
    });

    c.bench_function("day03 part 2", |b| {
        b.iter(|| day03::part2(black_box(&day03::get_input())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
