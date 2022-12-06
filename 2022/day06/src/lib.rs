use std::collections::HashSet;

pub fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
    println!("part two: {:?}", part2(&input));
}

pub fn get_input() -> String {
    include_str!("../../inputs/06.txt").replace("\r", "")
}

pub fn part1(input: &str) -> usize {
    process_window(input, 4)
}

pub fn part2(input: &str) -> usize {
    process_window(input, 14)
}

pub fn process_window(input: &str, window_size: usize) -> usize {
    let mut processed = 0;
    for four in input.as_bytes().windows(window_size) {
        let mut set = HashSet::new();
        if four.iter().all(|b| set.insert(b)) {
            break;
        }
        processed += 1;
    }
    processed + window_size
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TESTINPUT_1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const TESTINPUT_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const TESTINPUT_3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const TESTINPUT_4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const TESTINPUT_5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_part1_1() {
        let expected = 7;
        let actual = part1(TESTINPUT_1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_1() {
        let expected = 19;
        let actual = part2(TESTINPUT_1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_2() {
        let expected = 5;
        let actual = part1(TESTINPUT_2);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_2() {
        let expected = 23;
        let actual = part2(TESTINPUT_2);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_3() {
        let expected = 6;
        let actual = part1(TESTINPUT_3);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_3() {
        let expected = 23;
        let actual = part2(TESTINPUT_3);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_4() {
        let expected = 10;
        let actual = part1(TESTINPUT_4);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_4() {
        let expected = 29;
        let actual = part2(TESTINPUT_4);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_5() {
        let expected = 11;
        let actual = part1(TESTINPUT_5);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_5() {
        let expected = 26;
        let actual = part2(TESTINPUT_5);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected = 1175;
        let actual = part1(&get_input());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected = 3217;
        let actual = part2(&get_input());
        assert_eq!(actual, expected);
    }
}
