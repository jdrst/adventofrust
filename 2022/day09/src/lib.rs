use std::collections::HashSet;

pub fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
    println!("part two: {:?}", part2(&input));
}

pub fn get_input() -> String {
    include_str!("../../inputs/09.txt").replace("\r", "")
}

pub fn part1(input: &str) -> usize {
    let mut visited = HashSet::new();
    let mut head_pos = Point(0, 0);
    let mut tail_pos = Point(0, 0);
    for line in input.lines() {
        let (direction, amount) = line.split_once(" ").unwrap();
        let max = amount.parse::<usize>().unwrap();
        for _ in 0..max {
            head_pos.move_in(direction);
            tail_pos.move_to_if_necessary(&head_pos);
            visited.insert(tail_pos);
        }
    }
    visited.len()
}

pub fn part2(input: &str) -> usize {
    let mut visited = HashSet::new();
    let mut rope = vec![Point(0, 0); 10];
    for line in input.lines() {
        let (direction, amount) = line.split_once(" ").unwrap();
        let max = amount.parse::<usize>().unwrap();
        for _ in 0..max {
            rope[0].move_in(direction);
            let tail_pos = move_rope(&mut rope);
            visited.insert(tail_pos);
        }
    }
    visited.len()
}

fn move_rope(rope: &mut Vec<Point>) -> Point {
    for i in 1..rope.len() {
        let before = rope[i - 1];
        rope[i].move_to_if_necessary(&before);
    }
    rope[9]
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
struct Point(isize, isize);

impl Point {
    fn move_in(&mut self, direction: &str) {
        match direction {
            "U" => self.1 += 1,
            "D" => self.1 -= 1,
            "L" => self.0 -= 1,
            "R" => self.0 += 1,
            _ => unimplemented!("not a direction"),
        }
    }

    fn move_to_if_necessary(&mut self, other: &Point) {
        if !self.is_in_range_of(other) {
            // signum returns -1 for neg. numbers and +1 for pos. numbers and 0 for 0
            // that means
            //    if other.x is less than self.x
            //    we subtract 1 from self.x
            //    or else
            //    we add 1 to self.x
            // or in different words
            // we move self.x towards other.x
            self.0 = self.0 + (other.0 - self.0).signum();
            self.1 = self.1 + (other.1 - self.1).signum();
        }
    }

    fn is_in_range_of(self, other: &Point) -> bool {
        //if the difference between self.x and other.x is greater that 1 we are not in range
        (self.0 - other.0).abs() < 2 && (self.1 - other.1).abs() < 2
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TESTINPUT_1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const TESTINPUT_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_part1() {
        let expected = 13;
        let actual = part1(TESTINPUT_1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_1() {
        let expected = 1;
        let actual = part2(TESTINPUT_1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let expected = 36;
        let actual = part2(TESTINPUT_2);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected = 6090;
        let actual = part1(&get_input());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected = 2566;
        let actual = part2(&get_input());
        assert_eq!(actual, expected);
    }
}
