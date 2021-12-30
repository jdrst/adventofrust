use std::cmp;

fn main() {
    let input = include_str!("../../inputs/02.txt");

    let boxes: Vec<Vec<i32>> = parse_input(input);

    println!("part one: {:?}", part1(&boxes));
    println!("part two: {:?}", part2(&boxes));
}

fn part1(instructions: &[Vec<i32>]) -> usize {
    instructions.iter().fold(0, |mut paper, present| {
        let s1 = present[0] * present[1];
        let s2 = present[1] * present[2];
        let s3 = present[2] * present[0];
        paper += 2 * s1 + 2 * s2 + 2 * s3 + cmp::min(cmp::min(s1, s2), s3);
        paper
    }) as usize
}

fn part2(instructions: &[Vec<i32>]) -> usize {
    instructions.iter().fold(0, |mut ribbon, present| {
        let mut present = present.to_owned();
        present.sort();
        ribbon += present[0] * present[1] * present[2] + 2 * (present[0] + present[1]);
        ribbon
    }) as usize
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .split("\n")
        .map(|l| {
            l.to_string()
                .split("x")
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part1, part2};

    const TESTINPUT: &str = "2x3x4";

    #[test]
    fn test_part1() {
        let expected: usize = 58;
        let actual: usize = part1(&parse_input(TESTINPUT));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let expected: usize = 34;
        let actual: usize = part2(&parse_input(TESTINPUT));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected: usize = 1606483;
        let actual: usize = part1(&parse_input(include_str!("input.txt")));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected: usize = 3842356;
        let actual: usize = part2(&parse_input(include_str!("input.txt")));
        assert_eq!(actual, expected);
    }
}
