use std::str::FromStr;

fn main() {
    let input = include_str!("../../inputs/02.txt");

    let depths: Vec<Cmd> = parse_input(input);

    println!("part one: {:?}", part1(&depths));
    println!("part two: {:?}", part2(&depths));
}

struct Cmd {
    dir: Direction,
    amt: i32,
}

enum Direction {
    Forward,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => Err(()),
        }
    }
}

fn part1(commands: &[Cmd]) -> usize {
    let mut horizontal_position: i32 = 0;
    let mut vertical_position: i32 = 0;
    commands.iter().for_each(|cmd| match &cmd.dir {
        Direction::Forward => horizontal_position += cmd.amt,
        Direction::Up => vertical_position -= cmd.amt,
        Direction::Down => vertical_position += cmd.amt,
    });
    (horizontal_position * vertical_position) as usize
}

fn part2(commands: &[Cmd]) -> usize {
    let mut horizontal_position: i32 = 0;
    let mut vertical_position: i32 = 0;
    let mut aim: i32 = 0;
    commands.iter().for_each(|cmd| match &cmd.dir {
        Direction::Forward => {
            horizontal_position += cmd.amt;
            vertical_position += aim * cmd.amt
        }
        Direction::Up => aim -= cmd.amt,
        Direction::Down => aim += cmd.amt,
    });
    (horizontal_position * vertical_position) as usize
}

fn parse_input(input: &str) -> Vec<Cmd> {
    input
        .split("\n")
        .map(|l| l.split(' ').collect())
        .map(|vec: Vec<&str>| Cmd {
            dir: Direction::from_str(vec[0]).unwrap(),
            amt: vec[1].parse().unwrap(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part1, part2};

    const TESTINPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_part1() {
        let expected: usize = 150;
        let actual: usize = part1(&parse_input(TESTINPUT));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let expected: usize = 900;
        let actual: usize = part2(&parse_input(TESTINPUT));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected: usize = 1698735;
        let actual: usize = part1(&parse_input(include_str!("input.txt")));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected: usize = 1594785890;
        let actual: usize = part2(&parse_input(include_str!("input.txt")));
        assert_eq!(actual, expected);
    }
}
