fn main() {
    let input = parse_input(include_str!("../../inputs/04.txt"));

    println!("input: {}", input);
    println!("part one: {:?}", part(&input, 5));
    println!("part two: {:?}", part(&input, 6));
}

fn part(instructions: &String, num: usize) -> usize {
    let mut i = 0;
    while true {
        let s = format!("{}{}", instructions, i.to_string());
        let digest = format!("{:x}", md5::compute(s));
        if digest.chars().take(num).all(|c| c == '0') {
            return i;
        };
        i += 1;
    }
    0
}

fn parse_input(input: &str) -> String {
    input.replace("\r\n", "\n").replace("\n", "")
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part};

    const TESTINPUT: &str = "abcdef";
    const TESTINPUT2: &str = "pqrstuv";

    #[test]
    fn test_part1() {
        let expected: usize = 609043;
        let actual: usize = part(&parse_input(TESTINPUT), 5);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_1() {
        let expected: usize = 1048970;
        let actual: usize = part(&parse_input(TESTINPUT2), 5);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected: usize = 117946;
        let actual: usize = part(&parse_input(include_str!("input.txt")), 5);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected: usize = 3938038;
        let actual: usize = part(&parse_input(include_str!("input.txt")), 6);
        assert_eq!(actual, expected);
    }
}
