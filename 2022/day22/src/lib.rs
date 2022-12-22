use std::collections::HashSet;

pub fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
    println!("part two: {:?}", part2(&input));
}

pub fn get_input() -> String {
    include_str!("../../inputs/22.txt").replace("\r", "")
}

pub fn part1(input: &str) -> isize {
    let (floors, walls, directions, bounds) = parse_input(input);
    let mut position = (
        find_starting_position(&floors, bounds).unwrap(),
        Facing::Right,
    );

    for instr in directions {
        for _ in 0..instr.1 {
            let mut next_pos = match position.1 {
                Facing::Up => (position.0 .0, position.0 .1 - 1),
                Facing::Left => (position.0 .0 - 1, position.0 .1),
                Facing::Right => (position.0 .0 + 1, position.0 .1),
                Facing::Down => (position.0 .0, position.0 .1 + 1),
            };

            //wrap around
            while !walls.contains(&next_pos) && !floors.contains(&next_pos) {
                if next_pos.0 > bounds.0 {
                    next_pos.0 = 0;
                } else if next_pos.0 < 0 {
                    next_pos.0 = bounds.0;
                } else if next_pos.1 > bounds.1 {
                    next_pos.1 = 0;
                } else if next_pos.1 < 0 {
                    next_pos.1 = bounds.1;
                } else {
                    next_pos = match position.1 {
                        Facing::Up => (next_pos.0, next_pos.1 - 1),
                        Facing::Left => (next_pos.0 - 1, next_pos.1),
                        Facing::Right => (next_pos.0 + 1, next_pos.1),
                        Facing::Down => (next_pos.0, next_pos.1 + 1),
                    };
                }
            }

            if walls.contains(&next_pos) {
                continue;
            }

            if floors.contains(&next_pos) {
                position = (next_pos, position.1)
            }
        }
        position.1 = match (position.1, &instr.0) {
            (Facing::Up, Direction::Right) => Facing::Right,
            (Facing::Up, Direction::Left) => Facing::Left,
            (Facing::Left, Direction::Right) => Facing::Up,
            (Facing::Left, Direction::Left) => Facing::Down,
            (Facing::Right, Direction::Right) => Facing::Down,
            (Facing::Right, Direction::Left) => Facing::Up,
            (Facing::Down, Direction::Right) => Facing::Left,
            (Facing::Down, Direction::Left) => Facing::Right,
            (Facing::Up, Direction::None) => Facing::Up,
            (Facing::Left, Direction::None) => Facing::Left,
            (Facing::Right, Direction::None) => Facing::Right,
            (Facing::Down, Direction::None) => Facing::Down,
        };
    }
    1000 * (position.0 .1 + 1)
        + 4 * (position.0 .0 + 1)
        + match position.1 {
            Facing::Up => 3,
            Facing::Left => 2,
            Facing::Right => 0,
            Facing::Down => 1,
        }
}

pub fn part2(input: &str) -> isize {
    let (floors, walls, directions, bounds) = parse_input(input);
    let mut position = (
        find_starting_position(&floors, bounds).unwrap(),
        Facing::Right,
    );

    for instr in directions {
        for _ in 0..instr.1 {
            let mut next_pos = match position.1 {
                Facing::Up => (position.0 .0, position.0 .1 - 1),
                Facing::Left => (position.0 .0 - 1, position.0 .1),
                Facing::Right => (position.0 .0 + 1, position.0 .1),
                Facing::Down => (position.0 .0, position.0 .1 + 1),
            };

            let mut next_facing = position.1.clone();
            //wrap around
            if !walls.contains(&next_pos) && !floors.contains(&next_pos) {
                //Left/Right
                if position.1 == Facing::Left || position.1 == Facing::Right {
                    // 1 and 2
                    if next_pos.1 >= 0 && next_pos.1 < 50 {
                        if next_pos.0 < 50 {
                            //1 to left -> 4 from left
                            next_pos = (0, 149 - next_pos.1);
                            next_facing = Facing::Right;
                        } else {
                            //2 to right -> 5 from right
                            next_pos = (99, 149 - next_pos.1);
                            next_facing = Facing::Left;
                        }
                    } else
                    // 3
                    if next_pos.1 >= 50 && next_pos.1 < 100 {
                        if next_pos.0 < 50 {
                            // 3 to left -> 4 from top
                            next_pos = (next_pos.1 - 50, 100);
                            next_facing = Facing::Down;
                        } else {
                            //to right -> 2 from bottom
                            next_pos = (next_pos.1 + 50, 49);
                            next_facing = Facing::Up;
                        }
                    } else
                    // 4 and 5
                    if next_pos.1 >= 100 && next_pos.1 < 150 {
                        if next_pos.0 < 0 {
                            // 4 to left -> 1 from left
                            next_pos = (50, 149 - next_pos.1);
                            next_facing = Facing::Right;
                        } else {
                            // 5 to right -> 2 from right
                            next_pos = (149, 149 - next_pos.1);
                            next_facing = Facing::Left;
                        }
                    } else
                    // 6
                    if next_pos.1 >= 150 && next_pos.1 < 200 {
                        if next_pos.0 < 0 {
                            // 6 to left -> 1 from top
                            next_pos = (next_pos.1 - 100, 0);
                            next_facing = Facing::Down;
                        } else {
                            //to right -> 5 from bottom
                            next_pos = (next_pos.1 - 100, 149);
                            next_facing = Facing::Up;
                        }
                    }
                } else {
                    //Up/Down
                    // 4 and 6
                    if next_pos.0 >= 0 && next_pos.0 < 50 {
                        if next_pos.1 < 100 {
                            //4 to top -> 3 from left
                            next_pos = (50, next_pos.0 + 50);
                            next_facing = Facing::Right;
                        } else {
                            //6 to bottom -> 2 from top
                            next_pos = (next_pos.0 + 100, 0);
                            next_facing = Facing::Down;
                        }
                    } else
                    // 1, 3 and 5
                    if next_pos.0 >= 50 && next_pos.0 < 100 {
                        if next_pos.1 < 0 {
                            //1 to top -> 6 from left
                            next_pos = (0, next_pos.0 + 100);
                            next_facing = Facing::Right;
                        } else {
                            //5 to bottom -> 6 from right
                            next_pos = (49, next_pos.0 + 100);
                            next_facing = Facing::Left;
                        }
                    } else
                    // 2
                    if next_pos.0 >= 100 && next_pos.0 < 150 {
                        if next_pos.1 < 0 {
                            //2 to top -> 6 from bottom
                            next_pos = (next_pos.0 - 100, 199);
                            next_facing = Facing::Up;
                        } else {
                            //2 to bottom -> 3 from right
                            next_pos = (99, next_pos.0 - 50);
                            next_facing = Facing::Left;
                        }
                    }
                }
            }

            if walls.contains(&next_pos) {
                continue;
            }

            if floors.contains(&next_pos) {
                position = (next_pos, next_facing)
            }
        }
        position.1 = match (position.1, &instr.0) {
            (Facing::Up, Direction::Right) => Facing::Right,
            (Facing::Up, Direction::Left) => Facing::Left,
            (Facing::Left, Direction::Right) => Facing::Up,
            (Facing::Left, Direction::Left) => Facing::Down,
            (Facing::Right, Direction::Right) => Facing::Down,
            (Facing::Right, Direction::Left) => Facing::Up,
            (Facing::Down, Direction::Right) => Facing::Left,
            (Facing::Down, Direction::Left) => Facing::Right,
            (Facing::Up, Direction::None) => Facing::Up,
            (Facing::Left, Direction::None) => Facing::Left,
            (Facing::Right, Direction::None) => Facing::Right,
            (Facing::Down, Direction::None) => Facing::Down,
        };
    }
    1000 * (position.0 .1 + 1)
        + 4 * (position.0 .0 + 1)
        + match position.1 {
            Facing::Up => 3,
            Facing::Left => 2,
            Facing::Right => 0,
            Facing::Down => 1,
        }
}

fn find_starting_position(
    floors: &HashSet<(isize, isize)>,
    bounds: (isize, isize),
) -> Option<(isize, isize)> {
    for x in 0..bounds.0 {
        if floors.contains(&(x, 0)) {
            return Some((x, 0));
        }
    }
    None
}

#[allow(dead_code)]
fn print_map(
    floors: &HashSet<(isize, isize)>,
    walls: &HashSet<(isize, isize)>,
    (max_x, max_y): (isize, isize),
    position: &((isize, isize), Facing),
) -> String {
    let mut res = String::new();
    res.push_str(
        "--------------------------------------------------------------------------------\n\n",
    );
    for y in 0..=max_y {
        for x in 0..=max_x {
            if (x, y) == position.0 {
                match position.1 {
                    Facing::Up => res.push_str("^"),
                    Facing::Left => res.push_str("<"),
                    Facing::Right => res.push_str(">"),
                    Facing::Down => res.push_str("v"),
                };
            } else if floors.contains(&(x, y)) {
                res.push_str(".");
            } else if walls.contains(&(x, y)) {
                res.push_str("#");
            } else {
                res.push_str(" ");
            }
        }
        res.push_str("\n");
    }
    res.push_str(
        "\n\n--------------------------------------------------------------------------------\n",
    );
    res
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Facing {
    Up,
    Left,
    Right,
    Down,
}

#[derive(Debug)]
enum Direction {
    Right,
    Left,
    None,
}

fn parse_input(
    input: &str,
) -> (
    HashSet<(isize, isize)>,
    HashSet<(isize, isize)>,
    Vec<(Direction, isize)>,
    (isize, isize),
) {
    let mut max_x = 0isize;
    let mut max_y = 0isize;
    let (map, instructions) = input.split_once("\n\n").unwrap();
    let mut floors = HashSet::new();
    let mut walls = HashSet::new();
    for (y, line) in map.lines().enumerate() {
        for (x, tile) in line.chars().enumerate() {
            match tile {
                '.' => {
                    floors.insert((x as isize, y as isize));
                }
                '#' => {
                    walls.insert((x as isize, y as isize));
                }
                _ => (),
            }
            max_x = max_x.max(x as isize);
        }
        max_y = max_y.max(y as isize);
    }
    let mut directions = Vec::new();
    let mut buf = Vec::new();
    for c in instructions.chars() {
        match c {
            'R' => {
                directions.push((
                    Direction::Right,
                    buf.iter()
                        .collect::<String>()
                        .parse::<isize>()
                        .expect("couldn't parse movement instruction"),
                ));
                buf.clear()
            }
            'L' => {
                directions.push((
                    Direction::Left,
                    buf.iter()
                        .collect::<String>()
                        .parse::<isize>()
                        .expect("couldn't parse movement instruction"),
                ));
                buf.clear()
            }
            _ => buf.push(c),
        }
    }
    if buf.len() > 0 {
        directions.push((
            Direction::None,
            buf.iter()
                .collect::<String>()
                .parse::<isize>()
                .expect("couldn't parse movement instruction"),
        ));
    }

    (floors, walls, directions, (max_x, max_y))
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TESTINPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn test_part1() {
        let expected = 6032;
        let actual = part1(TESTINPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let expected = 5031;
        let actual = part2(TESTINPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected = 155060;
        let actual = part1(&get_input());
        assert_eq!(actual, expected);
    }

    // #[test]
    // fn test_part2_with_input() {
    //     let expected = 0;
    //     let actual = part2(&get_input());
    //     assert_eq!(actual, expected);
    // }
}
