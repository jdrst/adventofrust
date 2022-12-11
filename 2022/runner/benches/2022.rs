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

    c.bench_function("day04/runtime/part1", |b| {
        b.iter(|| day04::part1(black_box(&day04::get_input())))
    });

    c.bench_function("day04/runtime/part2", |b| {
        b.iter(|| day04::part2(black_box(&day04::get_input())))
    });

    c.bench_function("day05/runtime/part1", |b| {
        b.iter(|| day05::part1(black_box(&day05::get_input())))
    });

    c.bench_function("day05/runtime/part2", |b| {
        b.iter(|| day05::part2(black_box(&day05::get_input())))
    });

    c.bench_function("day06/runtime/part1", |b| {
        b.iter(|| day06::part1(black_box(&day06::get_input())))
    });

    c.bench_function("day06/runtime/part2", |b| {
        b.iter(|| day06::part2(black_box(&day06::get_input())))
    });

    c.bench_function("day07/runtime/part1", |b| {
        b.iter(|| day07::part1(black_box(&day07::get_input())))
    });

    c.bench_function("day07/runtime/part2", |b| {
        b.iter(|| day07::part2(black_box(&day07::get_input())))
    });

    c.bench_function("day08/runtime/part1", |b| {
        b.iter(|| day08::part1(black_box(&day08::get_input())))
    });

    c.bench_function("day08/runtime/part2", |b| {
        b.iter(|| day08::part2(black_box(&day08::get_input())))
    });

    c.bench_function("day09/runtime/part1", |b| {
        b.iter(|| day09::part1(black_box(&day09::get_input())))
    });

    c.bench_function("day09/runtime/part2", |b| {
        b.iter(|| day09::part2(black_box(&day09::get_input())))
    });

    c.bench_function("day10/runtime/part1", |b| {
        b.iter(|| day10::part1(black_box(&day10::get_input())))
    });

    c.bench_function("day10/runtime/part2", |b| {
        b.iter(|| day10::part2(black_box(&day10::get_input())))
    });

    c.bench_function("day11/runtime/part1", |b| {
        b.iter(|| day11::part1(black_box(&day11::get_input())))
    });

    c.bench_function("day11/runtime/part2", |b| {
        b.iter(|| day11::part2(black_box(&day11::get_input())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
