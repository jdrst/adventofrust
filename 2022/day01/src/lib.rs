pub fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
    println!("part two: {:?}", part2(&input));
}

fn get_input() -> String {
    include_str!("../../inputs/01.txt").replace("\r", "")
}

pub fn part1(input: &str) -> usize {
    parse_input(input).last().unwrap().to_owned()
}

pub fn part2(input: &str) -> usize {
    let cals = parse_input(input);
    let len = cals.len();
    cals[len - 1] + cals[len - 2] + cals[len - 3]
}

fn parse_input(input: &str) -> Vec<usize> {
    let mut cals: Vec<usize> = input
        .split("\n\n")
        .map(|l| {
            l.split("\n")
                .map(|s| s.trim().parse::<usize>().unwrap())
                .sum()
        })
        .collect();
    cals.sort();
    cals
}

#[cfg(test)]
mod tests {
    use crate::{get_input, part1, part2};

    const TESTINPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part1() {
        let expected: usize = 24000;
        let actual: usize = part1(TESTINPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let expected: usize = 45000;
        let actual: usize = part2(TESTINPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected: usize = 68923;
        let actual: usize = part1(&get_input());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected: usize = 200044;
        let actual: usize = part2(&get_input());
        assert_eq!(actual, expected);
    }
}
