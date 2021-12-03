use std::fs;

fn main() {
    let input = fs::read_to_string("01\\input.txt").expect("can't read file.");

    let depths: Vec<i32> = parse_input(input);

    println!("part one: {:?}", part1(&depths));
    println!("part two: {:?}", part2(&depths));
}

fn part1(depths: &[i32]) -> i32 {
    let mut prev: &i32 = &depths[0];
    let mut sum: i32 = 0;
    for i in &depths[1..] {
        if i > prev {
            sum += 1;
        }   
        prev = i;
    }
    return sum
}

fn part2(depths: &[i32]) -> i32 {
    let mut prev: &i32 = &depths[0];
    let mut sum: i32 = 0;
    for (pos, i)  in depths[2..].iter().enumerate() {
        if i > prev {
            sum += 1;
        }   
        prev = &depths[pos];
    }
    return sum
}

fn parse_input(input: String) -> Vec<i32> {
    input.split_whitespace().map(|l| l.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{part1, parse_input, part2};

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
        let expected: i32 = 7;
        let actual: i32 = part1(&parse_input(TESTINPUT.to_string()));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let expected: i32 = 6;
        let actual: i32 = part2(&parse_input(TESTINPUT.to_string()));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected: i32 = 1162;
        let actual: i32 = part2(&parse_input(fs::read_to_string("01\\input.txt").expect("can't read file.")));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected: i32 = 1190;
        let actual: i32 = part2(&parse_input(fs::read_to_string("01\\input.txt").expect("can't read file.")));
        assert_eq!(actual, expected);
    }
}