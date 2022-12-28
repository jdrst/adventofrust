use std::collections::HashSet;

pub fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
    println!("part two: {:?}", part2(&input));
}

pub fn get_input() -> String {
    include_str!("../../inputs/14.txt").replace("\r", "")
}

pub fn part1(input: &str) -> usize {
    let mut units = 0;
    let (mut spatial_cave, max_y) = parse_cave_spatial(input);
    while let Some(_) = do_thing(&mut spatial_cave, (500, 0), max_y) {
        units += 1;
    }
    units
}

pub fn part1_set(input: &str) -> usize {
    let mut units = 0;
    let (mut spatial_cave, max_y) = parse_cave_set(input);
    while let Some(_) = do_thing_set(&mut spatial_cave, (500, 0), max_y) {
        units += 1;
    }
    units
}

pub fn part2(input: &str) -> usize {
    let mut units = 0;
    let (mut cave, bottom) = parse_cave_spatial(input);
    let max_x = cave[0].len();
    let new_bottom = bottom + 2;
    cave.append(&mut vec![vec![false; max_x]]);
    cave.append(&mut vec![vec![true; max_x]]);

    while let Some((x, y)) = do_thing_with_grow(&mut cave, (500, 0), new_bottom) {
        units += 1;
        if x == 500 && y == 0 {
            break;
        }
    }
    units
}

fn do_thing(
    cave: &mut Vec<Vec<bool>>,
    (starting_x, starting_y): (usize, usize),
    bottom: usize,
) -> Option<()> {
    for next_y in starting_y..=bottom {
        if cave[next_y][starting_x] {
            //we hit. check left.
            if !cave[next_y][starting_x - 1] {
                return do_thing(cave, (starting_x - 1, next_y), bottom);
                //left free, go again
            } else if !cave[next_y][starting_x + 1] {
                //right free, go again
                return do_thing(cave, (starting_x + 1, next_y), bottom);
            } else {
                //nothing free. we stay here.
                cave[next_y - 1][starting_x] = true;
                return Some(());
            }
        }
    }
    None
}

fn do_thing_set(
    cave: &mut HashSet<(usize, usize)>,
    (starting_x, starting_y): (usize, usize),
    bottom: usize,
) -> Option<()> {
    for next_y in starting_y..=bottom {
        if cave.contains(&(starting_x, next_y)) {
            //we hit. check left.
            if !cave.contains(&(starting_x - 1, next_y)) {
                return do_thing_set(cave, (starting_x - 1, next_y), bottom);
                //left free, go again
            } else if !cave.contains(&(starting_x + 1, next_y)) {
                //right free, go again
                return do_thing_set(cave, (starting_x + 1, next_y), bottom);
            } else {
                //nothing free. we stay here.
                cave.insert((starting_x, next_y - 1));
                return Some(());
            }
        }
    }
    None
}

fn do_thing_with_grow(
    cave: &mut Vec<Vec<bool>>,
    (starting_x, starting_y): (usize, usize),
    bottom: usize,
) -> Option<(usize, usize)> {
    for next_y in starting_y..=bottom {
        if cave[next_y][starting_x] {
            //we hit. check left.
            if !cave[next_y][starting_x - 1] {
                return do_thing_with_grow(cave, (starting_x - 1, next_y), bottom);
                //left free, go again
            } else {
                //do we need to grow the array?
                if starting_x + 1 >= cave[next_y].len() {
                    //do grow
                    for y in 0..=bottom {
                        if y == bottom {
                            cave[y].append(&mut vec![true; 100]);
                        } else {
                            cave[y].append(&mut vec![false; 100]);
                        }
                    }
                }
                if !cave[next_y][starting_x + 1] {
                    //right free, go again
                    return do_thing_with_grow(cave, (starting_x + 1, next_y), bottom);
                } else {
                    //nothing free. we stay here.
                    cave[next_y - 1][starting_x] = true;
                    return Some((starting_x, next_y - 1));
                }
            }
        }
    }
    None
}

fn parse_tuple(p: &str) -> (usize, usize) {
    let (a, b) = p.split_once(",").unwrap();
    (
        a.parse::<usize>().expect("couldn't parse point x"),
        b.parse::<usize>().expect("couldn't parse point b"),
    )
}

fn parse_cave_spatial(input: &str) -> (Vec<Vec<bool>>, usize) {
    let mut stones = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for line in input.lines() {
        let mut points = line.split(" -> ");
        let (mut from, mut to) = parse_tuple(points.next().unwrap());
        for point in points {
            max_y = to.max(max_y);
            max_x = from.max(max_x);
            let (next_from, next_to) = parse_tuple(point);
            let min_from = from.min(next_from);
            let min_to = to.min(next_to);
            let max_from = from.max(next_from);
            let max_to = to.max(next_to);
            for x in min_from..=max_from {
                for y in min_to..=max_to {
                    stones.push((x, y))
                }
            }
            from = next_from;
            to = next_to;
        }
    }
    let mut cave = vec![vec![false; max_x + 1]; max_y + 1];
    for (x, y) in stones {
        cave[y][x] = true;
    }
    (cave, max_y)
}

fn parse_cave_set(input: &str) -> (HashSet<(usize, usize)>, usize) {
    let mut stones = HashSet::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for line in input.lines() {
        let mut points = line.split(" -> ");
        let (mut from, mut to) = parse_tuple(points.next().unwrap());
        for point in points {
            max_y = to.max(max_y);
            max_x = from.max(max_x);
            let (next_from, next_to) = parse_tuple(point);
            let min_from = from.min(next_from);
            let min_to = to.min(next_to);
            let max_from = from.max(next_from);
            let max_to = to.max(next_to);
            for x in min_from..=max_from {
                for y in min_to..=max_to {
                    stones.insert((x, y));
                }
            }
            from = next_from;
            to = next_to;
        }
    }
    (stones, max_y)
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TESTINPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part1() {
        let expected: usize = 24;
        let actual: usize = part1(TESTINPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_set() {
        let expected: usize = 24;
        let actual: usize = part1_set(TESTINPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let expected: usize = 93;
        let actual: usize = part2(TESTINPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected: usize = 779;
        let actual: usize = part1(&get_input());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_set_with_input() {
        let expected: usize = 779;
        let actual: usize = part1_set(&get_input());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected: usize = 27426;
        let actual: usize = part2(&get_input());
        assert_eq!(actual, expected);
    }
}
