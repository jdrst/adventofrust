pub fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
    println!("part two: {:?}", part2(&input));
}

fn get_input() -> String {
    include_str!("../../inputs/03.txt").replace("\r", "")
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| get_first_duplicate(l.split_at(l.len() / 2)))
        .map(|c| get_priority(c))
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .fold(0, |acc, sacks| {
            let badge = sacks[0]
                .chars()
                .find(|item| {
                    sacks[1..]
                        .iter()
                        .all(|sack| sack.chars().any(|other_item| other_item == *item))
                })
                .unwrap();
            acc + get_priority(badge)
        })
}

fn get_priority(c: char) -> usize {
    match c as u8 {
        41..=90 => c as usize - 38,
        61..=122 => c as usize - 96,
        _ => panic!("not an ASCII char in [a-zA-Z]"),
    }
}

fn get_first_duplicate((first, second): (&str, &str)) -> char {
    first
        .chars()
        .find(|c| second.chars().any(|b| b == *c))
        .unwrap()
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
