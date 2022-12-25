use std::collections::{BTreeMap, HashSet, VecDeque};

pub fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
    println!("part two: {:?}", part2(&input));
}

pub fn get_input() -> String {
    include_str!("../../inputs/24.txt").replace("\r", "")
}

pub fn part1(input: &str) -> usize {
    let (map, blizzards) = parse_input(input);
    let upper_bounds = (map[0].len() - 2, map.len() - 2);

    let target_pos = (upper_bounds.0, upper_bounds.1 + 1);
    let starting_pos = (1, 0);
    bfs_solve(starting_pos, target_pos, &upper_bounds, &map, blizzards).1 - 1
}

pub fn part2(input: &str) -> usize {
    let (map, blizzards) = parse_input(input);
    let upper_bounds = (map[0].len() - 2, map.len() - 2);

    let mut starting_pos = (1, 0);
    let mut target_pos = (upper_bounds.0, upper_bounds.1 + 1);
    let (blizzards, first) = bfs_solve(starting_pos, target_pos, &upper_bounds, &map, blizzards);

    starting_pos = (upper_bounds.0, upper_bounds.1 + 1);
    target_pos = (1, 0);
    let (blizzards, second) = bfs_solve(starting_pos, target_pos, &upper_bounds, &map, blizzards);

    starting_pos = (1, 0);
    target_pos = (upper_bounds.0, upper_bounds.1 + 1);
    let (_, third) = bfs_solve(starting_pos, target_pos, &upper_bounds, &map, blizzards);

    first + second + third - 1
}

fn bfs_solve(
    starting_pos: (usize, usize),
    target_pos: (usize, usize),
    upper_bounds: &(usize, usize),
    map: &Vec<Vec<char>>,
    mut blizzards: BTreeMap<(usize, usize), Vec<char>>,
) -> (BTreeMap<(usize, usize), Vec<char>>, usize) {
    let mut queue = VecDeque::new();
    let mut explored = HashSet::new();
    let mut current_step = 0;

    queue.push_back((starting_pos, 0));
    explored.insert((starting_pos, 0));

    while let Some(next) = queue.pop_front() {
        if next.1 == current_step {
            blizzards = move_blizzards(blizzards, upper_bounds);
            current_step += 1;
        }

        if next.0 == target_pos {
            break;
        }

        for after in adjacents(next.0, upper_bounds) {
            if !blizzards.contains_key(&after)
                && !explored.contains(&(after, next.1 + 1))
                && map[after.1][after.0] != '#'
            {
                queue.push_back((after, next.1 + 1));
                explored.insert((after, next.1 + 1));
            }
        }
    }

    (blizzards, current_step)
}

#[allow(dead_code)]
fn draw_map(
    current_pos: &(usize, usize),
    blizzards: &BTreeMap<(usize, usize), Vec<char>>,
    map: &Vec<Vec<char>>,
) -> String {
    let mut res = String::new();
    res += "\n-------------------------\n";
    for (y, line) in map.iter().enumerate() {
        for (x, pos) in line.iter().enumerate() {
            if &(x, y) == current_pos {
                res += "E"
            } else if let Some(zards) = blizzards.get(&(x, y)) {
                if zards.len() > 1 {
                    res += &format!["{}", zards.len()];
                } else {
                    res += &format!["{}", zards[0]];
                }
            } else {
                res += &format!["{}", pos];
            }
        }
        res += "\n";
    }
    res += "\n-------------------------\n";
    res
}

fn adjacents((x, y): (usize, usize), upper_bounds: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    if x > 0 {
        res.push((x - 1, y));
    }
    if y > 0 {
        res.push((x, y - 1));
    }
    if x < upper_bounds.0 + 1 {
        res.push((x + 1, y));
    }
    if y < upper_bounds.1 + 1 {
        res.push((x, y + 1));
    }
    res.push((x, y));
    res
}

fn move_blizzards(
    blizzards: BTreeMap<(usize, usize), Vec<char>>,
    upper_bounds: &(usize, usize),
) -> BTreeMap<(usize, usize), Vec<char>> {
    let mut new_blizzards: BTreeMap<(usize, usize), Vec<char>> = BTreeMap::new();
    for (pos, zards) in blizzards {
        for zard in zards {
            let new_pos = match zard {
                '>' => {
                    let mut next_pos = (pos.0 + 1, pos.1);
                    if next_pos.0 > upper_bounds.0 {
                        next_pos.0 = 1;
                    };
                    next_pos
                }
                'v' => {
                    let mut next_pos = (pos.0, pos.1 + 1);
                    if next_pos.1 > upper_bounds.1 {
                        next_pos.1 = 1;
                    };
                    next_pos
                }
                '<' => {
                    let mut next_pos = (pos.0 - 1, pos.1);
                    if next_pos.0 < 1 {
                        next_pos.0 = upper_bounds.0;
                    };
                    next_pos
                }
                '^' => {
                    let mut next_pos = (pos.0, pos.1 - 1);
                    if next_pos.1 < 1 {
                        next_pos.1 = upper_bounds.1;
                    };
                    next_pos
                }
                _ => unreachable!("this is not a blizzard."),
            };
            if let Some(zards) = new_blizzards.get_mut(&new_pos) {
                zards.push(zard);
            } else {
                new_blizzards.insert(new_pos, Vec::from_iter([zard]));
            }
        }
    }
    new_blizzards
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, BTreeMap<(usize, usize), Vec<char>>) {
    let mut blizzards: BTreeMap<(usize, usize), Vec<char>> = BTreeMap::new();
    let map = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '>' | '<' | 'v' | '^' => {
                        // if let Some(tile) = blizzards.get_mut(&(x, y)) {
                        //     tile.push(c);
                        // } else {
                        blizzards.insert((x, y), Vec::from_iter([c]));
                        // }
                        '.'
                    }
                    _ => c,
                })
                .collect()
        })
        .collect();
    (map, blizzards)
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TESTINPUT_1: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn test_part1() {
        let expected = 18;
        let actual = part1(TESTINPUT_1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let expected = 54;
        let actual = part2(TESTINPUT_1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected = 247;
        let actual = part1(&get_input());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected = 728;
        let actual = part2(&get_input());
        assert_eq!(actual, expected);
    }
}
