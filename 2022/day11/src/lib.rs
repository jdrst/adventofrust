use std::collections::VecDeque;

pub fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
    println!("part two: {:?}", part2(&input));
}

pub fn get_input() -> String {
    include_str!("../../inputs/11.txt").replace("\r", "")
}

pub fn part1(input: &str) -> usize {
    let mut monkeys = get_monkeys(input);
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let mut throws: Vec<(usize, usize)> = Vec::with_capacity(monkeys[i].items.len());
            let m = monkeys.get_mut(i).unwrap();
            while let Some(item) = m.items.pop_front() {
                m.inspects += 1;
                let new = m.op.execute(item) / 3;
                let throw_to = m.test(new);
                throws.push((throw_to, new))
            }
            for i in throws {
                let m = monkeys.get_mut(i.0).unwrap();
                m.items.push_back(i.1)
            }
        }
    }
    monkeys.sort_by(|a, b| b.inspects.partial_cmp(&a.inspects).unwrap());
    monkeys[0].inspects * monkeys[1].inspects
}

pub fn part2(input: &str) -> usize {
    let mut monkeys = get_monkeys(input);
    let div = monkeys.iter().map(|m| m.rem).product::<usize>();
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let mut throws: Vec<(usize, usize)> = Vec::with_capacity(monkeys[i].items.len());
            let m = monkeys.get_mut(i).unwrap();
            while let Some(item) = m.items.pop_front() {
                m.inspects += 1;
                let new = m.op.execute(item) % div; //chinese remainder theorem - we want a number that is smaller but leaves the same remainder
                let throw_to = m.test(new);
                throws.push((throw_to, new))
            }
            for i in throws {
                let m = monkeys.get_mut(i.0).unwrap();
                m.items.push_back(i.1)
            }
        }
    }
    monkeys.sort_by(|a, b| b.inspects.partial_cmp(&a.inspects).unwrap());
    monkeys[0].inspects * monkeys[1].inspects
}

enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

impl Operation {
    fn execute(&self, item: usize) -> usize {
        match self {
            Operation::Add(u) => u + item,
            Operation::Multiply(u) => u * item,
            Operation::Square => item * item,
        }
    }
}

struct Monkey {
    items: VecDeque<usize>,
    op: Operation,
    throws_to: (usize, usize),
    rem: usize,
    inspects: usize,
}

impl Monkey {
    fn test(&self, item: usize) -> usize {
        if item % self.rem == 0 {
            self.throws_to.0
        } else {
            self.throws_to.1
        }
    }
}

fn get_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey_str| parse_monkey(monkey_str))
        .collect()
}

fn parse_monkey(monkey_str: &str) -> Monkey {
    let lines = monkey_str.split("\n").collect::<Vec<&str>>();
    let starting_items = lines[1]
        .trim_start_matches("  Starting items: ")
        .split(",")
        .map(|item| {
            item.trim()
                .parse::<usize>()
                .expect("unable to parse starting items.")
        })
        .collect::<VecDeque<usize>>();
    let operation = lines[2]
        .trim_start_matches("  Operation: new = old ")
        .split_once(" ")
        .map(|v| {
            if v.0 == "*" && v.1 == "old" {
                return Operation::Square;
            }
            let num = v.1.parse::<usize>().unwrap();
            match v.0 {
                "+" => Operation::Add(num),
                "*" => Operation::Multiply(num),
                _ => unimplemented!("operator {} not implemented.", v.0),
            }
        })
        .expect("unable to parse operation.");
    let rem = lines[3]
        .trim_start_matches("  Test: divisible by ")
        .trim()
        .parse::<usize>()
        .expect("unable to parse remainder");
    let throws_to_true = lines[4]
        .trim_start_matches("    If true: throw to monkey ")
        .trim()
        .parse::<usize>()
        .expect("unable to parse throws true");
    let throws_to_false = lines[5]
        .trim_start_matches("    If false: throw to monkey ")
        .trim()
        .parse::<usize>()
        .expect("unable to parse throws false");
    Monkey {
        items: starting_items,
        op: operation,
        throws_to: (throws_to_true, throws_to_false),
        rem,
        inspects: 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TESTINPUT_1: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_part1() {
        let expected = 10605;
        let actual = part1(TESTINPUT_1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let expected = 2713310158;
        let actual = part2(TESTINPUT_1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected = 101436;
        let actual = part1(&get_input());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected = 19754471646;
        let actual = part2(&get_input());
        assert_eq!(actual, expected);
    }
}
