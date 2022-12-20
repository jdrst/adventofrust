pub fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
    println!("part two: {:?}", part2(&input));
}

pub fn get_input() -> String {
    include_str!("../../inputs/20.txt").replace("\r", "")
}

pub fn part1(input: &str) -> isize {
    let mut numbers = parse_input(input);
    let length = numbers.len();

    let zero_idx = mix_numbers(&mut numbers, 1);

    numbers[(zero_idx + 1000) % length].value
        + numbers[(zero_idx + 2000) % length].value
        + numbers[(zero_idx + 3000) % length].value
}

pub fn part2(input: &str) -> isize {
    let mut numbers = parse_input(input);
    let length = numbers.len();
    numbers.iter_mut().for_each(|n| n.value *= 811589153);

    let zero_idx = mix_numbers(&mut numbers, 10);

    numbers[(zero_idx + 1000) % length].value
        + numbers[(zero_idx + 2000) % length].value
        + numbers[(zero_idx + 3000) % length].value
}

fn mix_numbers(numbers: &mut Vec<Number>, times: usize) -> usize {
    for _ in 0..times {
        let mut next_in_order = 0;
        loop {
            if next_in_order == numbers.len() {
                break;
            }

            let i = numbers
                .iter()
                .position(|n| n.position == next_in_order)
                .expect("couldn't find next in order.");

            let element = numbers.remove(i);

            let mut new_index = i as isize + element.value;
            new_index %= numbers.len() as isize;
            if new_index <= 0 {
                new_index = (numbers.len() as isize) + new_index;
            }

            numbers.insert(new_index as usize, element);

            next_in_order += 1;
        }
    }

    numbers.iter().position(|n| n.value == 0).unwrap()
}

fn parse_input(input: &str) -> Vec<Number> {
    input
        .lines()
        .enumerate()
        .map(|(i, l)| Number {
            value: l.parse::<isize>().expect("can't parse number"),
            position: i,
        })
        .collect()
}

#[derive(Clone, Debug)]
struct Number {
    value: isize,
    position: usize,
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TESTINPUT_1: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn test_part1() {
        let expected = 3;
        let actual = part1(TESTINPUT_1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let expected = 1623178306;
        let actual = part2(TESTINPUT_1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected = 7004;
        let actual = part1(&get_input());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected = 17200008919529;
        let actual = part2(&get_input());
        assert_eq!(actual, expected);
    }
}
