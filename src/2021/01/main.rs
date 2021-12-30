fn main() {
    let input = include_str!("../../inputs/01.txt");

    let depths: Vec<i32> = parse_input(input);

    println!("part one: {:?}", part1(&depths));
    println!("part two: {:?}", part2(&depths));
}

fn part1(depths: &[i32]) -> usize {
    depths.windows(2).filter(|pair| pair[0] < pair[1]).count()
}

fn part2(depths: &[i32]) -> usize {
    depths.windows(4).filter(|pair| pair[0] < pair[3]).count()
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .map(|l| l.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part1, part2};

    const TESTINPUT: &str = "199
    200
    208
    210
    200
    207
    240
    269
    260
    263";

    #[test]
    fn test_part1() {
        let expected: usize = 7;
        let actual: usize = part1(&parse_input(TESTINPUT));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let expected: usize = 5;
        let actual: usize = part2(&parse_input(TESTINPUT));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected: usize = 1162;
        let actual: usize = part1(&parse_input(include_str!("input.txt")));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected: usize = 1190;
        let actual: usize = part2(&parse_input(include_str!("input.txt")));
        assert_eq!(actual, expected);
    }
}
