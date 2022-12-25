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

    c.bench_function("day12/runtime/part1", |b| {
        b.iter(|| day12::part1(black_box(&day12::get_input())))
    });

    c.bench_function("day12/runtime/part2", |b| {
        b.iter(|| day12::part2(black_box(&day12::get_input())))
    });

    c.bench_function("day13/runtime/part1", |b| {
        b.iter(|| day13::part1(black_box(&day13::get_input())))
    });

    c.bench_function("day13/runtime/part2", |b| {
        b.iter(|| day13::part2(black_box(&day13::get_input())))
    });

    c.bench_function("day14/runtime/part1", |b| {
        b.iter(|| day14::part1(black_box(&day14::get_input())))
    });

    c.bench_function("day14/runtime/part2", |b| {
        b.iter(|| day14::part2(black_box(&day14::get_input())))
    });

    c.bench_function("day15/runtime/part1", |b| {
        b.iter(|| day15::part1(black_box(&day15::get_input()), black_box(2_000_000)))
    });

    c.bench_function("day15/runtime/part2", |b| {
        b.iter(|| day15::part2(black_box(&day15::get_input()), black_box(4_000_000)))
    });

    c.bench_function("day16/runtime/part1", |b| {
        b.iter(|| day16::part1(black_box(&day16::get_input())))
    });

    c.bench_function("day16/runtime/part2", |b| {
        b.iter(|| day16::part2(black_box(&day16::get_input())))
    });

    c.bench_function("day17/runtime/part1", |b| {
        b.iter(|| day17::part1(black_box(&day17::get_input())))
    });

    c.bench_function("day17/runtime/part2", |b| {
        b.iter(|| day17::part2(black_box(&day17::get_input())))
    });

    c.bench_function("day18/runtime/part1", |b| {
        b.iter(|| day18::part1(black_box(&day18::get_input())))
    });

    c.bench_function("day18/runtime/part2", |b| {
        b.iter(|| day18::part2(black_box(&day18::get_input())))
    });

    c.bench_function("day19/runtime/part1", |b| {
        b.iter(|| day19::part1(black_box(&day19::get_input())))
    });

    c.bench_function("day19/runtime/part2", |b| {
        b.iter(|| day19::part2(black_box(&day19::get_input())))
    });

    c.bench_function("day20/runtime/part1", |b| {
        b.iter(|| day20::part1(black_box(&day20::get_input())))
    });

    c.bench_function("day20/runtime/part2", |b| {
        b.iter(|| day20::part2(black_box(&day20::get_input())))
    });

    c.bench_function("day21/runtime/part1", |b| {
        b.iter(|| day21::part1(black_box(&day21::get_input())))
    });

    c.bench_function("day21/runtime/part2", |b| {
        b.iter(|| day21::part2(black_box(&day21::get_input())))
    });

    c.bench_function("day22/runtime/part1", |b| {
        b.iter(|| day22::part1(black_box(&day22::get_input())))
    });

    // c.bench_function("day22/runtime/part2", |b| {
    //     b.iter(|| day22::part2(black_box(&day22::get_input())))
    // });

    c.bench_function("day23/runtime/part1", |b| {
        b.iter(|| day23::part1(black_box(&day23::get_input())))
    });

    c.bench_function("day23/runtime/part2", |b| {
        b.iter(|| day23::part2(black_box(&day23::get_input())))
    });

    c.bench_function("day24/runtime/part1", |b| {
        b.iter(|| day24::part1(black_box(&day24::get_input())))
    });

    c.bench_function("day24/runtime/part2", |b| {
        b.iter(|| day24::part2(black_box(&day24::get_input())))
    });

    c.bench_function("day25/runtime/part1", |b| {
        b.iter(|| day25::part1(black_box(&day25::get_input())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
