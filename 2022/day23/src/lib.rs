use std::collections::{HashMap, HashSet, VecDeque};

pub fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
    println!("part two: {:?}", part2(&input));
}

pub fn get_input() -> String {
    include_str!("../../inputs/23.txt").replace("\r", "")
}

pub fn part1(input: &str) -> usize {
    let mut elves = parse_input(input);
    let mut adjacent_order = VecDeque::from_iter([(0usize, 1, 2), (5, 6, 7), (0, 3, 5), (2, 4, 7)]);

    for _ in 1..=10 {
        let mut proposed_positions = HashMap::<(isize, isize), isize>::new();
        let mut planned_moves = HashMap::<(isize, isize), (isize, isize)>::new();
        elves.iter().for_each(|elf| {
            let adjacents = get_adjacent(elf);
            if !adjacents.iter().all(|adj| !elves.contains(adj)) {
                for (f, s, t) in &adjacent_order {
                    if !elves.contains(&adjacents[*f])
                        && !elves.contains(&adjacents[*s])
                        && !elves.contains(&adjacents[*t])
                    {
                        let new_pos = adjacents[*s];
                        let amount = proposed_positions.get(&new_pos).unwrap_or(&0);
                        proposed_positions.insert(new_pos, amount + 1);
                        planned_moves.insert(*elf, new_pos);
                        break;
                    }
                }
            }
        });
        planned_moves.iter().for_each(|(elf, next)| {
            if proposed_positions.get(next).unwrap() < &2 {
                elves.remove(elf);
                elves.insert(*next);
            }
        });
        let new_back = adjacent_order.pop_front().unwrap();
        adjacent_order.push_back(new_back);
    }

    count_free_spaces(&elves)
}

pub fn part2(input: &str) -> isize {
    let mut elves = parse_input(input);
    let mut adjacent_order = VecDeque::from_iter([(0usize, 1, 2), (5, 6, 7), (0, 3, 5), (2, 4, 7)]);
    // let mut adjacent_order = [(0usize, 1, 2), (5, 6, 7), (0, 3, 5), (2, 4, 7)];
    let mut has_moved = true;
    let mut i = 0;
    while has_moved {
        has_moved = false;

        let mut planned_moves = HashMap::<(isize, isize), (isize, isize)>::new();

        elves.iter().for_each(|elf| {
            let adjacents = get_adjacent(elf);
            if !adjacents.iter().all(|adj| !elves.contains(adj)) {
                for (f, s, t) in &adjacent_order {
                    if !elves.contains(&adjacents[*f])
                        && !elves.contains(&adjacents[*s])
                        && !elves.contains(&adjacents[*t])
                    {
                        let new_pos = adjacents[*s];
                        if planned_moves.contains_key(&new_pos) {
                            planned_moves.remove(&new_pos);
                        } else {
                            planned_moves.insert(new_pos, *elf);
                        }
                        break;
                    }
                }
            }
        });

        planned_moves.iter().for_each(|(elf, next)| {
            elves.remove(next);
            elves.insert(*elf);
            has_moved = true;
        });
        let new_back = adjacent_order.pop_front().unwrap();
        adjacent_order.push_back(new_back);
        i += 1;
    }

    i
}

fn count_free_spaces(elves: &HashSet<(isize, isize)>) -> usize {
    let mut y_min = isize::MAX;
    let mut y_max = isize::MIN;
    let mut x_min = isize::MAX;
    let mut x_max = isize::MIN;
    let mut count = 0;
    elves.iter().for_each(|e| {
        y_min = y_min.min(e.1);
        y_max = y_max.max(e.1);
        x_min = x_min.min(e.0);
        x_max = x_max.max(e.0);
        count += 1;
    });
    (y_min.abs_diff(y_max) + 1) * (x_min.abs_diff(x_max) + 1) - count
}

fn draw_elves(elves: &HashSet<(isize, isize)>) {
    let mut y_min = isize::MAX;
    let mut y_max = isize::MIN;
    let mut x_min = isize::MAX;
    let mut x_max = isize::MIN;
    let mut count = 0;
    elves.iter().for_each(|e| {
        y_min = y_min.min(e.1);
        y_max = y_max.max(e.1);
        x_min = x_min.min(e.0);
        x_max = x_max.max(e.0);
        count += 1;
    });

    println!["\n----------"];
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if elves.contains(&(x, y)) {
                print!["#"];
            } else {
                print!["."];
            };
        }
        println![];
    }
    println!["----------\n"];
}

fn get_adjacent((x, y): &(isize, isize)) -> [(isize, isize); 8] {
    [
        (*x - 1, *y - 1),
        (*x, *y - 1),
        (*x + 1, *y - 1),
        (*x - 1, *y),
        (*x + 1, *y),
        (*x - 1, *y + 1),
        (*x, *y + 1),
        (*x + 1, *y + 1),
    ]
}

fn parse_input(input: &str) -> HashSet<(isize, isize)> {
    let mut res = HashSet::new();
    input.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| match c {
            '#' => {
                res.insert((x as isize, y as isize));
            }
            _ => (),
        })
    });
    res
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TESTINPUT: &str = "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";

    #[test]
    fn test_part1() {
        let expected = 110;
        let actual = part1(TESTINPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let expected = 20;
        let actual = part2(TESTINPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected = 3800;
        let actual = part1(&get_input());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected = 916;
        let actual = part2(&get_input());
        assert_eq!(actual, expected);
    }
}
