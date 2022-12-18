use std::collections::HashSet;

pub fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
    println!("part two: {:?}", part2(&input));
}

pub fn get_input() -> String {
    include_str!("../../inputs/18.txt").replace("\r", "")
}

pub fn part1(input: &str) -> usize {
    let droplets = parse_droplets(input);
    let mut surface_area = 0;
    for droplet in &droplets {
        surface_area += droplet.adjacent_positions().iter().fold(0, |acc, adj| {
            if !droplets.contains(adj) {
                acc + 1
            } else {
                acc
            }
        })
    }
    surface_area
}

pub fn part2(input: &str) -> usize {
    let droplets = parse_droplets(input);
    let enclosed_air = get_enclosed_air_bubbles(&droplets);
    let mut surface_area = 0;

    for droplet in &droplets {
        surface_area += droplet.adjacent_positions().iter().fold(0, |acc, adj| {
            if !droplets.contains(adj) {
                acc + 1
            } else {
                acc
            }
        })
    }

    for bubble in &enclosed_air {
        surface_area -= bubble.adjacent_positions().iter().fold(0, |acc, adj| {
            if !enclosed_air.contains(adj) {
                acc + 1
            } else {
                acc
            }
        })
    }

    surface_area
}

fn has_droplet_in_all_directions(
    position: &Position,
    droplets: &HashSet<Position>,
    (max_x, max_y, max_z): (isize, isize, isize),
) -> bool {
    let x = position.0;
    let y = position.1;
    let z = position.2;

    let mut is_enclosed_x_plus = false;
    let mut is_enclosed_x_minus = false;
    for x_plus in x..=max_x {
        if droplets.contains(&Position(x_plus, y, z)) {
            is_enclosed_x_plus = true;
            break;
        }
    }
    for x_minus in -1..x {
        if droplets.contains(&Position(x_minus, y, z)) {
            is_enclosed_x_minus = true;
            break;
        }
    }

    let mut is_enclosed_y_plus = false;
    let mut is_enclosed_y_minus = false;
    for y_plus in y..=max_y {
        if droplets.contains(&Position(x, y_plus, z)) {
            is_enclosed_y_plus = true;
            break;
        }
    }
    for y_minus in -1..y {
        if droplets.contains(&Position(x, y_minus, z)) {
            is_enclosed_y_minus = true;
            break;
        }
    }

    let mut is_enclosed_z_plus = false;
    let mut is_enclosed_z_minus = false;
    for z_plus in z..=max_z {
        if droplets.contains(&Position(x, y, z_plus)) {
            is_enclosed_z_plus = true;
            break;
        }
    }
    for z_minus in -1..z {
        if droplets.contains(&Position(x, y, z_minus)) {
            is_enclosed_z_minus = true;
            break;
        }
    }
    is_enclosed_x_plus
        && is_enclosed_x_minus
        && is_enclosed_y_minus
        && is_enclosed_y_plus
        && is_enclosed_z_minus
        && is_enclosed_z_plus
}

fn fill_possible_enclosing(
    maybe_res: &mut HashSet<Position>,
    position: &Position,
    droplets: &HashSet<Position>,
    (max_x, max_y, max_z): (isize, isize, isize),
) -> Option<()> {
    maybe_res.insert(position.clone());

    for pos in position.adjacent_positions() {
        if maybe_res.contains(&pos) {
            continue;
        }
        if pos.0 < 0 || pos.1 < 0 || pos.2 < 0 || pos.0 > max_x || pos.1 > max_y || pos.2 > max_z {
            return None;
        }
        if !droplets.contains(&pos) {
            maybe_res.insert(pos.clone());

            if let None = fill_possible_enclosing(maybe_res, &pos, droplets, (max_x, max_y, max_z))
            {
                return None;
            }
        }
    }

    Some(())
}

fn get_enclosed_air_bubbles(droplets: &HashSet<Position>) -> HashSet<Position> {
    let mut res = HashSet::new();
    let max_x = droplets.iter().fold(0, |acc, d| d.0.max(acc));
    let max_y = droplets.iter().fold(0, |acc, d| d.1.max(acc));
    let max_z = droplets.iter().fold(0, |acc, d| d.2.max(acc));
    for x in 0..max_x {
        for y in 0..max_y {
            for z in 0..max_z {
                let pos = Position(x, y, z);
                if !droplets.contains(&pos) && !res.contains(&pos) {
                    if has_droplet_in_all_directions(&pos, &droplets, (max_x, max_y, max_z)) {
                        let mut enclosed_positions = HashSet::new();
                        if let Some(_) = fill_possible_enclosing(
                            &mut enclosed_positions,
                            &pos,
                            droplets,
                            (max_x, max_y, max_z),
                        ) {
                            for bubble in enclosed_positions {
                                res.insert(bubble);
                            }
                        }
                    }
                }
            }
        }
    }
    res
}

fn parse_droplets(input: &str) -> HashSet<Position> {
    input
        .lines()
        .map(|l| {
            let mut sides = l.split(",");
            Position::new(
                sides
                    .next()
                    .unwrap()
                    .parse::<isize>()
                    .expect("can't parse x pos"),
                sides
                    .next()
                    .unwrap()
                    .parse::<isize>()
                    .expect("can't parse y pos"),
                sides
                    .next()
                    .unwrap()
                    .parse::<isize>()
                    .expect("can't parse z pos"),
            )
        })
        .collect::<HashSet<Position>>()
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Position(isize, isize, isize);

impl Position {
    fn adjacent_positions(&self) -> Vec<Self> {
        let mut res = Vec::new();
        res.push(Self::new(self.0 - 1, self.1, self.2));
        res.push(Self::new(self.0, self.1 - 1, self.2));
        res.push(Self::new(self.0, self.1, self.2 - 1));
        res.push(Self::new(self.0 + 1, self.1, self.2));
        res.push(Self::new(self.0, self.1 + 1, self.2));
        res.push(Self::new(self.0, self.1, self.2 + 1));
        res
    }

    fn new(x: isize, y: isize, z: isize) -> Self {
        Self(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TESTINPUT_1: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_part1() {
        let expected = 64;
        let actual = part1(TESTINPUT_1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let expected = 58;
        let actual = part2(TESTINPUT_1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected = 3390;
        let actual = part1(&get_input());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected = 2058;
        let actual = part2(&get_input());
        assert_eq!(actual, expected);
    }
}
