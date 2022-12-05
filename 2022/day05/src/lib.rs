pub fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
    println!("part two: {:?}", part2(&input));
}

pub fn get_input() -> String {
    include_str!("../../inputs/05.txt").replace("\r", "")
}

pub fn part1(input: &str) -> String {
    let (crates, procedures) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(crates);
    procedures
        .lines()
        .map(|l| Move::from_line(l))
        .for_each(|m| {
            (0..m.amount).for_each(|_| {
                let from = stacks[m.from - 1].pop().expect("stack empty.");
                stacks[m.to - 1].push(from);
            })
        });
    stacks.iter_mut().map(|s| s.pop().unwrap_or(' ')).collect()
}

pub fn part2(input: &str) -> String {
    let (crates, procedures) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(crates);
    procedures
        .lines()
        .map(|l| Move::from_line(l))
        .for_each(|m| {
            let amount = stacks[m.from - 1].len() - m.amount..;
            let crates: Vec<char> = stacks[m.from - 1].drain(amount).collect();
            stacks[m.to - 1].extend(crates);
        });
    stacks.iter_mut().map(|s| s.pop().unwrap_or(' ')).collect()
}

pub fn parse_stacks(crates: &str) -> Vec<Vec<char>> {
    let width = (crates.lines().rev().next().unwrap().len() + 1) / 4;
    let mut stacks = Vec::with_capacity(width);
    let cap = crates.lines().count() - 1;
    for _ in 0..width {
        stacks.push(Vec::with_capacity(cap))
    }
    crates.lines().rev().skip(1).for_each(|l| {
        let mut crates = l.chars().skip(1).step_by(4);
        stacks.iter_mut().for_each(|e| {
            if let Some(c) = crates.next() {
                if c.is_alphabetic() {
                    e.push(c)
                }
            }
        });
    });
    stacks
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
        Self {
            amount: split.nth(1).unwrap().parse::<usize>().unwrap(),
            from: split.nth(1).unwrap().parse::<usize>().unwrap(),
            to: split.nth(1).unwrap().parse::<usize>().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TESTINPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part1() {
        let expected = "CMZ".to_string();
        let actual = part1(TESTINPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let expected = "MCD".to_string();
        let actual = part2(TESTINPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected = "HBTMTBSDC".to_string();
        let actual = part1(&get_input());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected = "PQTJRSHWS".to_string();
        let actual = part2(&get_input());
        assert_eq!(actual, expected);
    }
}
