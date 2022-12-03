use std::collections::HashSet;

fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
    println!("part two: {:?}", part2(&input));
}

fn get_input() -> String {
    include_str!("../../inputs/03.txt").replace("\r", "")
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| get_duplicates(l))
        .map(|db| db.iter().map(|c| get_priority(c)).sum::<usize>())
        .sum()
}

fn part2(input: &str) -> usize {
    let groups = input
        .lines()
        .map(|l| {
            let mut chars = HashSet::new();
            for c in l.chars() {
                chars.insert(c);
            }
            chars
        })
        .collect::<Vec<HashSet<char>>>();
    let mut res = 0;
    for group in groups.chunks(3) {
        let badge: HashSet<&char> = group[0]
            .iter()
            .filter(|e| group[1..].iter().all(|other| other.contains(*e)))
            .collect();
        res += get_priority(*badge.iter().last().unwrap());
    }
    res
}

fn get_priority(c: &char) -> usize {
    match *c as u8 {
        41..=90 => *c as usize - 38,
        61..=122 => *c as usize - 96,
        _ => panic!("not an ascii char"),
    }
}

fn get_duplicates(line: &str) -> HashSet<char> {
    let mid = line.len() / 2;
    let first: HashSet<char> = line.chars().take(mid).collect();
    let second: HashSet<char> = line.chars().skip(mid).collect();
    // first.intersection(&second).cloned().collect()
    first.into_iter().filter(|c| second.contains(c)).collect()
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TESTINPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        let expected: usize = 157;
        let actual: usize = part1(TESTINPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let expected: usize = 70;
        let actual: usize = part2(TESTINPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected: usize = 8493;
        let actual: usize = part1(&get_input());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected: usize = 2552;
        let actual: usize = part2(&get_input());
        assert_eq!(actual, expected);
    }
}
