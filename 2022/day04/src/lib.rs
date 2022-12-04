use std::{collections::HashSet, hash::Hash, ops::Range};

pub fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
    println!("part two: {:?}", part2(&input));
}

pub fn get_input() -> String {
    include_str!("../../inputs/04.txt").replace("\r", "")
}

pub fn part1(input: &str) -> usize {
    input.lines().fold(0, |acc, l| {
        let (first, second) = l.split_once(',').unwrap();
        let a = create_set(first);
        let b = create_set(second);

        if contains(&a, &b) || contains(&b, &a) {
            return acc + 1;
        };
        acc
    })
}

pub fn part2(input: &str) -> usize {
    input.lines().fold(0, |acc, l| {
        let (first, second) = l.split_once(',').unwrap();
        let a = create_set(first);
        let b = create_set(second);

        if a.intersection(&b).count() > 0 {
            return acc + 1;
        };
        acc
    })
}

fn contains<T: Eq + Hash>(first: &HashSet<T>, second: &HashSet<T>) -> bool {
    for a in first {
        if !second.contains(&a) {
            return false;
        }
    }
    return true;
}

fn create_set(s: &str) -> HashSet<usize> {
    if let Some((start, end)) = s.split_once('-') {
        Range {
            start: start.parse::<usize>().unwrap(),
            end: end.parse::<usize>().unwrap() + 1,
        }
        .collect::<HashSet<usize>>()
    } else {
        let mut set = HashSet::new();
        set.insert(s.parse::<usize>().unwrap());
        set
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TESTINPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() {
        let expected: usize = 2;
        let actual: usize = part1(TESTINPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let expected: usize = 4;
        let actual: usize = part2(TESTINPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected: usize = 503;
        let actual: usize = part1(&get_input());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected: usize = 827;
        let actual: usize = part2(&get_input());
        assert_eq!(actual, expected);
    }
}
