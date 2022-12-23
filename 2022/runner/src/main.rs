use std::{collections::BTreeMap, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let fns = BTreeMap::<&str, fn() -> ()>::from([
        ("01", day01::main as fn()),
        ("02", day02::main),
        ("03", day03::main),
        ("04", day04::main),
        ("05", day05::main),
        ("06", day06::main),
        ("07", day07::main),
        ("08", day08::main),
        ("09", day09::main),
        ("10", day10::main),
        ("11", day11::main),
        ("12", day12::main),
        ("13", day13::main),
        ("14", day14::main),
        ("15", day15::main),
        ("17", day17::main),
        ("18", day18::main),
        ("19", day19::main),
        ("20", day20::main),
        ("21", day21::main),
        ("23", day23::main),
    ]);
    if args.len() == 1 {
        fns.iter().for_each(|(k, v)| {
            println!["day {k}"];
            v()
        })
    }
    if args.len() > 1 {
        if let Some(f) = fns.get(args[1].as_str()) {
            f();
        } else {
            println!["day {} not implemented", args[1]]
        }
    }
}
