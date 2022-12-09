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
