pub fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
    println!("part two: {:?}", part2(&input));
}

pub fn get_input() -> String {
    include_str!("../../inputs/04.txt").replace("\r", "")
}

pub fn part1(input: &str) -> usize {
    input
        .as_bytes()
        .split(|&b| b == b'\n')
        .filter(|pair| is_fully_containing(pair))
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .as_bytes()
        .split(|&b| b == b'\n')
        .filter(|pair| is_overlapping(pair))
        .count()
}

fn is_fully_containing(pair: &[u8]) -> bool {
    let sections: Vec<u128> = pair
        .split(|&b| b == b',')
        .map(|elf| get_section(elf))
        .collect();
    sections[0] & sections[1] == sections[0] || sections[0] & sections[1] == sections[1]
}

fn is_overlapping(pair: &[u8]) -> bool {
    pair.split(|&b| b == b',')
        .map(|elf| get_section(elf))
        .fold(u128::MAX, |overlap, elf| overlap & elf)
        > 0
}

fn get_section(e: &[u8]) -> u128 {
    if e.contains(&b'-') {
        let mut res = 0u128;
        let bounds: Vec<usize> = e.split(|&b| b == b'-').map(|e| to_integer(e)).collect();
        for i in bounds[0]..=bounds[1] {
            res = res | 1u128 << i
        }
        res
    } else {
        1u128 << to_integer(e)
    }
}

fn to_integer(bytes: &[u8]) -> usize {
    bytes
        .iter()
        .fold(0, |acc, b| (acc * 10) + (b - b'0') as usize)
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TESTINPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() {
        let expected: usize = 2;
        let actual: usize = part1(TESTINPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let expected: usize = 4;
        let actual: usize = part2(TESTINPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected: usize = 503;
        let actual: usize = part1(&get_input());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected: usize = 827;
        let actual: usize = part2(&get_input());
        assert_eq!(actual, expected);
    }
}
