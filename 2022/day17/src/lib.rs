use std::collections::HashMap;

pub fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
    println!("part two: {:?}", part2(&input));
}

pub fn get_input() -> String {
    include_str!("../../inputs/17.txt").replace("\r", "")
}

pub fn part1(input: &str) -> usize {
    simulate_cave_bit(input, 2022)
}

pub fn part2(input: &str) -> usize {
    simulate_cave_bit(input, 1_000_000_000_000)
}

fn get_rocks() -> Vec<Vec<u8>> {
    vec![
        vec![0b1_1110],
        vec![0b0_1000, 0b1_1100, 0b0_1000],
        vec![0b1_1100, 0b0_0100, 0b0_0100],
        vec![0b1_0000, 0b1_0000, 0b1_0000, 0b1_0000],
        vec![0b1_1000, 0b1_1000],
    ]
}

fn is_chamber_high_enough(chamber: &Vec<u8>, rock_height: usize) -> bool {
    if chamber.len() < 3 + rock_height {
        return false;
    }
    chamber.iter().rev().take(3 + rock_height).all(|l| l == &0)
}

fn move_left_if_possible(current_rock: &mut Vec<u8>, current_rock_pos: usize, chamber: &Vec<u8>) {
    if current_rock.iter().any(|l| l & 0b100_0000 > 0)
        || current_rock
            .iter()
            .enumerate()
            .any(|(i, l)| chamber[current_rock_pos + i] & l << 1 > 0)
    {
        return;
    }
    current_rock.iter_mut().for_each(|l| *l = *l << 1);
}

fn move_right_if_possible(current_rock: &mut Vec<u8>, current_rock_pos: usize, chamber: &Vec<u8>) {
    if current_rock.iter().any(|l| l & 0b000_0001 > 0)
        || current_rock
            .iter()
            .enumerate()
            .any(|(i, l)| chamber[current_rock_pos + i] & l >> 1 > 0)
    {
        return;
    }
    current_rock.iter_mut().for_each(|l| *l = *l >> 1);
}

fn has_collision_bottom(
    current_rock: &Vec<u8>,
    current_rock_pos: usize,
    chamber: &Vec<u8>,
) -> bool {
    current_rock
        .iter()
        .enumerate()
        .any(|(i, r)| chamber[current_rock_pos - 1 + i] & r > 0)
}

fn simulate_cave_bit(input: &str, until_rocks_stopped: usize) -> usize {
    let mut chamber = vec![0b000_0000, 0b000_0000, 0b000_0000, 0b000_0000];

    let mut jet_pattern = input.chars().enumerate().cycle();
    let jet_pattern_len = input.chars().count();
    let rocks = get_rocks();
    let mut rocks_cycle = rocks.iter().cycle();

    let mut current_rock = rocks_cycle.next().unwrap().clone();
    let mut rocks_cycle_pos = 1;
    let mut current_chamber_height = 0usize;
    let mut current_rock_pos = current_chamber_height + 3;
    let mut rocks_stopped = 0;
    let mut known_cycle_pos = HashMap::<(usize, usize, Vec<u8>), (usize, usize)>::new();
    let mut additional_chamber_height = 0;

    while rocks_stopped < until_rocks_stopped {
        let (jet_pattern_cycle_pos, pattern) = jet_pattern.next().unwrap();
        match pattern {
            '<' => move_left_if_possible(&mut current_rock, current_rock_pos, &chamber),
            '>' => move_right_if_possible(&mut current_rock, current_rock_pos, &chamber),
            _ => unreachable!("this shouldn't happen."),
        }

        if current_rock_pos == 0 || has_collision_bottom(&current_rock, current_rock_pos, &chamber)
        {
            current_chamber_height += current_rock.len()
                - current_rock
                    .len()
                    .min(current_chamber_height.abs_diff(current_rock_pos));
            current_rock.iter().enumerate().for_each(|(i, r)| {
                chamber[current_rock_pos + i] = chamber[current_rock_pos + i] | r
            });
            rocks_stopped += 1;

            current_rock = rocks_cycle.next().unwrap().clone();
            rocks_cycle_pos += 1;
            current_rock_pos = current_chamber_height + 3;
            while !is_chamber_high_enough(&chamber, current_rock.len()) {
                chamber.push(0b000_0000);
            }

            if let Some(hit) = known_cycle_pos.get(&(
                rocks_cycle_pos % rocks.len(),
                jet_pattern_cycle_pos % jet_pattern_len,
                chamber.iter().rev().take(20).cloned().collect(),
            )) {
                let rocks_diff_between_cycles = rocks_stopped - hit.0;
                let chamber_height_diff_between_cycles = current_chamber_height - hit.1;

                let mult = (until_rocks_stopped - rocks_stopped) / rocks_diff_between_cycles;

                rocks_stopped += rocks_diff_between_cycles * mult;
                additional_chamber_height += chamber_height_diff_between_cycles * mult;
            }

            known_cycle_pos.insert(
                (
                    rocks_cycle_pos % rocks.len(),
                    jet_pattern_cycle_pos % jet_pattern_len,
                    chamber.iter().rev().take(20).cloned().collect(),
                ),
                (rocks_stopped, current_chamber_height),
            );
        } else {
            current_rock_pos -= 1;
        }
    }

    current_chamber_height + additional_chamber_height
}

fn draw_bit_chamber(current_rock: &Vec<u8>, current_rock_pos: usize, chamber: &Vec<u8>) {
    let rock_height = current_rock.len();
    println![];
    println!["-------"];
    for (pos, mut line) in chamber.iter().enumerate().rev() {
        let mut next_line: u8;
        for i in 0..rock_height {
            if pos == current_rock_pos + i {
                next_line = line | current_rock[i];
                line = &next_line;
            }
        }
        for _ in 0..7 {
            let mut to_print = '.';
            if line & 0b100_0000 > 0 {
                to_print = '#';
            }
            next_line = line << 1;
            line = &next_line;
            print!["{to_print}"];
        }
        println![];
    }
    println!["-------"];
    println![];
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TESTINPUT_1: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_part1() {
        let expected = 3068;
        let actual = part1(TESTINPUT_1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let expected = 1_514_285_714_288;
        let actual = part2(TESTINPUT_1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected = 3069;
        let actual = part1(&get_input());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected = 1523167155404;
        let actual = part2(&get_input());
        assert_eq!(actual, expected);
    }
}
