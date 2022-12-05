pub fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
    println!("part two: {:?}", part2(&input));
}

pub fn get_input() -> String {
    include_str!("../../inputs/05.txt").replace("\r", "")
}

#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn from_line(line: &str) -> Self {
        let mut split = line.split(" ");
        let amount = split.nth(1).unwrap().parse::<usize>().unwrap();
        let from = split.nth(1).unwrap().parse::<usize>().unwrap();
        let to = split.nth(1).unwrap().parse::<usize>().unwrap();
        return Self { amount, from, to };
    }
}

pub fn part1(input: &str) -> String {
    let (crates, procedures) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(crates);
    procedures
        .lines()
        .map(|l| Move::from_line(l))
        .for_each(|m| {
            for _ in 0..m.amount {
                let from = stacks[m.from - 1].pop().expect("stack empty.");
                stacks[m.to - 1].push(from);
            }
        });
    let mut top = String::new();
    for i in 0..9 {
        if let Some(c) = stacks[i].pop() {
            top.push(c)
        } else {
            top.push(' ')
        }
    }
    top
}

pub fn parse_stacks(crates: &str) -> Vec<Vec<char>> {
    let mut stacks = Vec::with_capacity(9);
    for _ in 0..9 {
        stacks.push(Vec::with_capacity(20))
    }
    for line in crates.lines().rev().skip(1) {
        let mut j = 1;
        for k in 0..9 {
            match line.chars().nth(j) {
                Some(' ') => (),
                Some(c) => stacks[k].push(c),
                None => panic!("Oh no!"),
            }
            j += 4
        }
    }
    stacks
}

pub fn part2(input: &str) -> String {
    let (crates, procedures) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(crates);
    procedures
        .lines()
        .map(|l| Move::from_line(l))
        .for_each(|m| {
            let mut between = Vec::new();
            for _ in 0..m.amount {
                let from = stacks[m.from - 1].pop().expect("stack empty.");
                between.push(from);
            }
            for _ in 0..m.amount {
                let from = between.pop().expect("stack empty.");
                stacks[m.to - 1].push(from);
            }
        });
    let mut top = String::new();
    for i in 0..9 {
        if let Some(c) = stacks[i].pop() {
            top.push(c)
        } else {
            top.push(' ')
        }
    }
    top
}

// #[cfg(test)]
// mod tests {
//     use crate::*;

//     const TESTINPUT: &str = "2-4,6-8
// 2-3,4-5
// 5-7,7-9
// 2-8,3-7
// 6-6,4-6
// 2-6,4-8";

//     #[test]
//     fn test_part1() {
//         let expected: usize = 2;
//         let actual: usize = part1(TESTINPUT);
//         assert_eq!(actual, expected);
//     }

//     #[test]
//     fn test_part2() {
//         let expected: usize = 4;
//         let actual: usize = part2(TESTINPUT);
//         assert_eq!(actual, expected);
//     }

//     #[test]
//     fn test_part1_with_input() {
//         let expected: usize = 503;
//         let actual: usize = part1(&get_input());
//         assert_eq!(actual, expected);
//     }

//     #[test]
//     fn test_part2_with_input() {
//         let expected: usize = 827;
//         let actual: usize = part2(&get_input());
//         assert_eq!(actual, expected);
//     }
// }
