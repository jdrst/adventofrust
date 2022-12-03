use std::collections::HashMap;

fn main() {
    let input = include_str!("../../inputs/03.txt");

    let directions: Vec<char> = parse_input(input);

    println!("part one: {:?}", part1(&directions));
    println!("part two: {:?}", part2(&directions));
}

fn part1(instructions: &[char]) -> usize {
    let mut pos_y: i32 = 0;
    let mut pos_x: i32 = 0;
    let mut houses = HashMap::<(i32, i32), i32>::new();
    houses.insert((0, 0), 1);
    instructions.iter().for_each(|c| {
        match c {
            'v' => pos_y -= 1,
            '^' => pos_y += 1,
            '>' => pos_x += 1,
            '<' => pos_x -= 1,
            _ => (),
        };
        *houses.entry((pos_x, pos_y)).or_insert(0) += 1;
    });
    houses.len()
}

fn part2(instructions: &[char]) -> usize {
    let mut santa_pos_y: i32 = 0;
    let mut santa_pos_x: i32 = 0;
    let mut robosanta_pos_y: i32 = 0;
    let mut robosanta_pos_x: i32 = 0;
    let mut houses = HashMap::<(i32, i32), i32>::new();
    houses.insert((0, 0), 1);
    instructions.chunks(2).for_each(|w| {
        match w[0] {
            'v' => santa_pos_y -= 1,
            '^' => santa_pos_y += 1,
            '>' => santa_pos_x += 1,
            '<' => santa_pos_x -= 1,
            _ => (),
        };
        if w.len() > 1 {
            match w[1] {
                'v' => robosanta_pos_y -= 1,
                '^' => robosanta_pos_y += 1,
                '>' => robosanta_pos_x += 1,
                '<' => robosanta_pos_x -= 1,
                _ => (),
            };
        };
        *houses.entry((santa_pos_x, santa_pos_y)).or_insert(0) += 1;
        *houses
            .entry((robosanta_pos_x, robosanta_pos_y))
            .or_insert(0) += 1;
    });
    houses.len()
}

fn parse_input(input: &str) -> Vec<char> {
    input.chars().collect()
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part1, part2};

    const TESTINPUT: &str = "^>v<";

    #[test]
    fn test_part1() {
        let expected: usize = 4;
        let actual: usize = part1(&parse_input(TESTINPUT));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let expected: usize = 3;
        let actual: usize = part2(&parse_input(TESTINPUT));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected: usize = 2565;
        let actual: usize = part1(&parse_input(include_str!("input.txt")));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected: usize = 2639;
        let actual: usize = part2(&parse_input(include_str!("input.txt")));
        assert_eq!(actual, expected);
    }
}
