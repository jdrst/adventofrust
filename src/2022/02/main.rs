use std::cmp::Ordering;

#[derive(Eq, PartialEq, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

struct Match(RPS, RPS);

fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
    println!("part two: {:?}", part2(&input));
}

fn part1(input: &str) -> usize {
    parse_input1(input)
        .iter()
        .fold(0, |acc, m| m.score() + m.1.score() + acc)
}

fn part2(input: &str) -> usize {
    parse_input2(input)
        .iter()
        .fold(0, |acc, m| m.score() + m.1.score() + acc)
}

fn parse_input1(input: &str) -> Vec<Match> {
    input
        .split("\n")
        .map(|l| {
            Match(
                RPS::from_char(l.chars().nth(0).unwrap()),
                RPS::from_char(l.chars().nth(2).unwrap()),
            )
        })
        .collect()
}

fn parse_input2(input: &str) -> Vec<Match> {
    input
        .split("\n")
        .map(|l| {
            let elf = RPS::from_char(l.chars().nth(0).unwrap());
            let me = match l.chars().nth(2).unwrap() {
                'Y' => elf.clone(),
                'X' => elf.lose(),
                'Z' => elf.win(),
                _ => panic!("Oh no."),
            };
            Match(elf, me)
        })
        .collect()
}

impl Match {
    fn score(&self) -> usize {
        match self.0.cmp(&self.1) {
            Ordering::Less => 6,
            Ordering::Equal => 3,
            Ordering::Greater => 0,
        }
    }
}

impl RPS {
    fn from_char(input: char) -> Self {
        match input {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("Sorry, only classic rules!"),
        }
    }

    fn score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn win(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn lose(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }
}

impl Ord for RPS {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Rock, Self::Paper) => Ordering::Less,
            (Self::Rock, Self::Rock) => Ordering::Equal,
            (Self::Rock, Self::Scissors) => Ordering::Greater,
            (Self::Paper, Self::Scissors) => Ordering::Less,
            (Self::Paper, Self::Paper) => Ordering::Equal,
            (Self::Paper, Self::Rock) => Ordering::Greater,
            (Self::Scissors, Self::Rock) => Ordering::Less,
            (Self::Scissors, Self::Scissors) => Ordering::Equal,
            (Self::Scissors, Self::Paper) => Ordering::Greater,
        }
    }
}

impl PartialOrd for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_input() -> String {
    include_str!("../../inputs/02.txt").replace("\r", "")
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TESTINPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_part1() {
        let expected: usize = 15;
        let actual: usize = part1(TESTINPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let expected: usize = 12;
        let actual: usize = part2(TESTINPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected: usize = 10310;
        let actual: usize = part1(&get_input());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected: usize = 14859;
        let actual: usize = part2(&get_input());
        assert_eq!(actual, expected);
    }
}
