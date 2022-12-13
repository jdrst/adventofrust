use std::{cmp::Ordering, collections::VecDeque};

pub fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
    println!("part two: {:?}", part2(&input));
}

pub fn get_input() -> String {
    include_str!("../../inputs/13.txt").replace("\r", "")
}

pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .into_iter()
        .fold((1, 0), |(idx, ordered), pair| {
            let packets: Vec<_> = pair
                .lines()
                .map(|p| parse_packet(&mut tokenize(p)).unwrap())
                .collect();
            if packets[0] < packets[1] {
                (idx + 1, ordered + idx)
            } else {
                (idx + 1, ordered)
            }
        })
        .1
}

pub fn part2(input: &str) -> usize {
    let mut lns = input
        .replace("\n\n", "\n")
        .lines()
        .map(|l| parse_packet(&mut tokenize(l)).unwrap())
        .collect::<Vec<_>>();
    let two = Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]);
    let six = Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]);
    lns.push(two.clone());
    lns.push(six.clone());

    lns.sort();

    let two_idx = 1 + lns.iter().position(|p| p == &two).unwrap();
    let six_idx = 1 + lns.iter().position(|p| p == &six).unwrap();
    two_idx * six_idx
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Packet {
    Integer(usize),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        return match (self, other) {
            (Packet::Integer(i), Packet::Integer(j)) => i.cmp(j),
            (Packet::List(f), Packet::List(s)) => f.iter().cmp(s),
            (Packet::Integer(i), Packet::List(_)) => {
                Packet::List(vec![Packet::Integer(*i)]).cmp(other)
            }
            (Packet::List(_), Packet::Integer(j)) => {
                self.cmp(&Packet::List(vec![Packet::Integer(*j)]))
            }
        };
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn tokenize(line: &str) -> VecDeque<String> {
    line.replace(',', " ")
        .replace("[", " [ ")
        .replace("]", " ] ")
        .split_whitespace()
        .map(|c| String::from(c))
        .collect()
}

fn parse_packet(tokens: &mut VecDeque<String>) -> Option<Packet> {
    for token in tokens.pop_front() {
        match token.as_str() {
            "[" => {
                let mut list = Vec::new();
                while tokens.front().expect("incomplete list") != "]" {
                    list.push(parse_packet(tokens).expect("unable to parse tokens"))
                }
                tokens.pop_front().expect("missing ]");
                return Some(Packet::List(list));
            }
            _ => {
                let int = token.parse::<usize>().expect("unable to parse integer");
                return Some(Packet::Integer(int));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TESTINPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_part1() {
        let expected: usize = 13;
        let actual: usize = part1(TESTINPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let expected: usize = 140;
        let actual: usize = part2(TESTINPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected: usize = 6415;
        let actual: usize = part1(&get_input());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected: usize = 20056;
        let actual: usize = part2(&get_input());
        assert_eq!(actual, expected);
    }
}
