use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day01/runtime/part1", |b| {
        b.iter(|| day01::part1(black_box(&day01::get_input())))
    });

    c.bench_function("day01/runtime/part2", |b| {
        b.iter(|| day01::part2(black_box(&day01::get_input())))
    });

    c.bench_function("day02/runtime/part1", |b| {
        b.iter(|| day02::part1(black_box(&day02::get_input())))
    });

    c.bench_function("day02/runtime/part2", |b| {
        b.iter(|| day02::part2(black_box(&day02::get_input())))
    });

    c.bench_function("day03/runtime/part1", |b| {
        b.iter(|| day03::part1(black_box(&day03::get_input())))
    });

    c.bench_function("day03/runtime/part2", |b| {
        b.iter(|| day03::part2(black_box(&day03::get_input())))
    });

    c.bench_function("day03/runtime/part1", |b| {
        b.iter(|| day04::part1(black_box(&day04::get_input())))
    });

    c.bench_function("day04/runtime/part2", |b| {
        b.iter(|| day04::part2(black_box(&day04::get_input())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
