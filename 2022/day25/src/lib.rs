pub fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
}

pub fn get_input() -> String {
    include_str!("../../inputs/25.txt").replace("\r", "")
}

pub fn part1(input: &str) -> String {
    to_snafu_number(input.lines().map(|n| parse_snafu_number(n)).sum())
}

fn parse_snafu_number(number: &str) -> isize {
    number
        .chars()
        .rev()
        .enumerate()
        .map(|(i, d)| {
            let factor = match d {
                '1' => 1,
                '2' => 2,
                '0' => 0,
                '=' => -2,
                '-' => -1,
                _ => unreachable!("not a snafu number"),
            };
            factor * 5isize.pow(i as u32)
        })
        .sum()
}

fn to_snafu_number(mut number: isize) -> String {
    let mut res = String::new();
    let mut carry = 0;
    while number > 0 {
        match (number + carry) % 5 {
            4 => {
                carry = 1;
                res.push_str("-");
            }
            3 => {
                carry = 1;
                res.push_str("=");
            }
            2 => {
                carry = 0;
                res.push_str("2");
            }
            1 => {
                carry = 0;
                res.push_str("1");
            }
            0 => {
                res.push_str("0");
            }
            _ => unreachable!("this shouldn't happen."),
        };
        number = number / 5;
    }
    if carry > 0 {
        res.push_str("1");
    }

    res.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TESTINPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn test_part1() {
        let expected = "2=-1=0";
        let actual = part1(TESTINPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_snafu_number_1() {
        let expected = "1";
        let actual = to_snafu_number(1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_snafu_number_2() {
        let expected = "2";
        let actual = to_snafu_number(2);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_snafu_number_3() {
        let expected = "1=";
        let actual = to_snafu_number(3);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_snafu_number_4() {
        let expected = "1-";
        let actual = to_snafu_number(4);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_snafu_number_5() {
        let expected = "10";
        let actual = to_snafu_number(5);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_snafu_number_6() {
        let expected = "11";
        let actual = to_snafu_number(6);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_snafu_number_7() {
        let expected = "12";
        let actual = to_snafu_number(7);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_snafu_number_8() {
        let expected = "2=";
        let actual = to_snafu_number(8);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_snafu_number_9() {
        let expected = "2-";
        let actual = to_snafu_number(9);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_snafu_number_10() {
        let expected = "20";
        let actual = to_snafu_number(10);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_snafu_number_11() {
        let expected = "21";
        let actual = to_snafu_number(11);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_snafu_number_12() {
        let expected = "22";
        let actual = to_snafu_number(12);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_snafu_number_13() {
        let expected = "1==";
        let actual = to_snafu_number(13);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_snafu_number_14() {
        let expected = "1=-";
        let actual = to_snafu_number(14);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_snafu_number_15() {
        let expected = "1=0";
        let actual = to_snafu_number(15);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_snafu_number_16() {
        let expected = "1=1";
        let actual = to_snafu_number(16);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_snafu_number_17() {
        let expected = "1=2";
        let actual = to_snafu_number(17);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_snafu_number_18() {
        let expected = "1-=";
        let actual = to_snafu_number(18);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_snafu_number_19() {
        let expected = "1--";
        let actual = to_snafu_number(19);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_snafu_number_20() {
        let expected = "1-0";
        let actual = to_snafu_number(20);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_snafu_number_21() {
        let expected = "1-1";
        let actual = to_snafu_number(21);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_snafu_number_22() {
        let expected = "1-2";
        let actual = to_snafu_number(22);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_snafu_number_23() {
        let expected = "10=";
        let actual = to_snafu_number(23);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_snafu_number_24() {
        let expected = "10-";
        let actual = to_snafu_number(24);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_snafu_number_25() {
        let expected = "100";
        let actual = to_snafu_number(25);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_snafu_number_2022() {
        let expected = "1=11-2";
        let actual = to_snafu_number(2022);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_snafu_number_314159265() {
        let expected = "1121-1110-1=0";
        let actual = to_snafu_number(314159265);
        assert_eq!(actual, expected);
    }
}
