use std::collections::HashSet;

pub fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
    println!("part two: {:?}", part2(&input));
}

pub fn get_input() -> String {
    include_str!("../../inputs/08.txt").replace("\r", "")
}

pub fn part1(input: &str) -> usize {
    let max_rows = input.lines().count();
    let mut cols = Vec::with_capacity(max_rows);

    for line in input.lines() {
        cols.push(line.chars().collect::<Vec<_>>());
    }

    let mut visible = HashSet::<(usize, usize)>::new();
    for i in 0..max_rows {
        let mut highest_left = b'0' - 1;
        let mut highest_right = b'0' - 1;
        let mut highest_top = '0' as u8 - 1;
        let mut highest_bot = '0' as u8 - 1;
        for k in 0..cols[i].len() {
            let j = cols[i].len() - 1 - k;
            if highest_left < cols[i][k] as u8 {
                highest_left = cols[i][k] as u8;
                visible.insert((i, k));
            }
            if highest_right < cols[i][j] as u8 {
                highest_right = cols[i][j] as u8;
                visible.insert((i, j));
            }
            let j = max_rows - 1 - k;
            if highest_top < cols[k][i] as u8 {
                highest_top = cols[k][i] as u8;
                visible.insert((k, i));
            }
            if highest_bot < cols[j][i] as u8 {
                highest_bot = cols[j][i] as u8;
                visible.insert((j, i));
            }
        }
    }
    visible.len()
}

pub fn part2(input: &str) -> usize {
    let max_rows = input.lines().count();
    let mut cols = Vec::with_capacity(max_rows);

    for line in input.lines() {
        cols.push(line.chars().collect::<Vec<_>>());
    }

    let mut highest_scenic_score = 0;
    for i in 0..max_rows {
        for k in 0..cols[i].len() {
            let current = cols[i][k] as u8;
            let mut left_score = 0;
            for j in (0..k).rev() {
                left_score += 1;
                if current <= cols[i][j] as u8 {
                    break;
                }
            }
            let mut right_score = 0;
            for j in k + 1..max_rows {
                right_score += 1;
                if current <= cols[i][j] as u8 {
                    break;
                }
            }
            let mut top_score = 0;
            for j in (0..i).rev() {
                top_score += 1;
                if current <= cols[j][k] as u8 {
                    break;
                }
            }
            let mut bottom_score = 0;
            for j in i + 1..cols[i].len() {
                bottom_score += 1;
                if current <= cols[j][k] as u8 {
                    break;
                }
            }
            let scenic_score = left_score * right_score * bottom_score * top_score;
            if scenic_score > highest_scenic_score {
                highest_scenic_score = scenic_score;
            }
        }
    }
    highest_scenic_score
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TESTINPUT_1: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part1() {
        let expected = 21;
        let actual = part1(TESTINPUT_1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let expected = 8;
        let actual = part2(TESTINPUT_1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected = 1796;
        let actual = part1(&get_input());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected = 288120;
        let actual = part2(&get_input());
        assert_eq!(actual, expected);
    }
}
