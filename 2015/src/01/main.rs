fn main() {
    let input = include_str!("../../inputs/01.txt");

    let instructions: Vec<char> = parse_input(input);

    println!("part one: {:?}", part1(&instructions));
    println!("part two: {:?}", part2(&instructions));
}

fn part1(instructions: &[char]) -> isize {
    instructions.iter().fold(0, |mut floor, c| {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        };
        floor
    })
}

fn part2(instructions: &[char]) -> usize {
    let mut idx = 0;
    let mut floor = 0;
    for c in instructions.iter() {
        idx += 1;
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        };
        if floor < 0 {
            break;
        }
    }
    idx
}

fn parse_input(input: &str) -> Vec<char> {
    input.chars().collect()
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part1, part2};

    const TESTINPUT: &str = ")())())";

    #[test]
    fn test_part1() {
        let expected: isize = -3;
        let actual: isize = part1(&parse_input(TESTINPUT));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let expected: usize = 1;
        let actual: usize = part2(&parse_input(TESTINPUT));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected: isize = 74;
        let actual: isize = part1(&parse_input(include_str!("input.txt")));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected: usize = 1795;
        let actual: usize = part2(&parse_input(include_str!("input.txt")));
        assert_eq!(actual, expected);
    }
}
