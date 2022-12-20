use std::collections::HashSet;

pub fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
    println!("part two: {:?}", part2(&input));
}

pub fn get_input() -> String {
    include_str!("../../inputs/19.txt").replace("\r", "")
}

pub fn part1(input: &str) -> usize {
    let factories = parse_input(input);
    factories
        .iter()
        .map(|f| produce_until(24, f.clone(), 0, None, &mut HashSet::new()) * f.blueprint.id)
        .sum()
}

pub fn part2(input: &str) -> usize {
    let factories = parse_input(input);
    factories
        .iter()
        .take(3)
        .map(|f| produce_until(32, f.clone(), 0, None, &mut HashSet::new()))
        .product()
}

fn produce_until(
    time_limit: usize,
    mut f: Factory,
    mut current_minute: usize,
    robot_to_build: Option<Material>,
    state: &mut HashSet<(
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
    )>,
) -> usize {
    current_minute += 1;
    f.harvest();

    match robot_to_build {
        Some(Material::Ore) => {
            f.ore_robots += 1;
            f.ore_count -= f.blueprint.ore_robot_cost;
        }
        Some(Material::Clay) => {
            f.clay_robots += 1;
            f.ore_count -= f.blueprint.clay_robot_cost;
        }
        Some(Material::Obsidian) => {
            f.obsidian_robots += 1;
            f.ore_count -= f.blueprint.obsidian_robot_cost.0;
            f.clay_count -= f.blueprint.obsidian_robot_cost.1;
        }
        Some(Material::Geode) => {
            f.geode_robots += 1;
            f.ore_count -= f.blueprint.geode_robot_cost.0;
            f.obsidian_count -= f.blueprint.geode_robot_cost.1;
        }
        None => (),
    };

    if state.contains(&f.get_state(current_minute)) {
        return 0;
    } else {
        state.insert(f.get_state(current_minute));
    }
    if current_minute >= time_limit {
        return f.geode_count;
    }

    if f.has_geode_robot_cost() {
        return produce_until(time_limit, f, current_minute, Some(Material::Geode), state);
    }

    let try_obsidian_robot = {
        if f.has_obsidian_robot_cost() && f.blueprint.geode_robot_cost.1 > f.obsidian_robots {
            Some(produce_until(
                time_limit,
                f,
                current_minute,
                Some(Material::Obsidian),
                state,
            ))
        } else {
            None
        }
    };

    let try_clay_robot = {
        if f.has_clay_robot_cost() && f.blueprint.obsidian_robot_cost.1 > f.clay_robots {
            Some(produce_until(
                time_limit,
                f,
                current_minute,
                Some(Material::Clay),
                state,
            ))
        } else {
            None
        }
    };

    let try_ore_robot = {
        if f.has_ore_robot_cost() && f.highest_ore_cost() > f.ore_robots {
            Some(produce_until(
                time_limit,
                f,
                current_minute,
                Some(Material::Ore),
                state,
            ))
        } else {
            None
        }
    };

    let just_harvest = { Some(produce_until(time_limit, f, current_minute, None, state)) };

    [
        try_obsidian_robot,
        try_ore_robot,
        try_clay_robot,
        just_harvest,
    ]
    .iter()
    .filter_map(|s| *s)
    .max()
    .unwrap()
}

#[derive(Debug, Clone, Copy)]
struct Factory {
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
    ore_count: usize,
    clay_count: usize,
    obsidian_count: usize,
    geode_count: usize,

    blueprint: Blueprint,
}

impl Factory {
    fn harvest(&mut self) {
        self.ore_count += self.ore_robots;
        self.clay_count += self.clay_robots;
        self.obsidian_count += self.obsidian_robots;
        self.geode_count += self.geode_robots;
    }

    fn highest_ore_cost(&self) -> usize {
        self.blueprint
            .ore_robot_cost
            .max(self.blueprint.clay_robot_cost)
            .max(self.blueprint.obsidian_robot_cost.0)
            .max(self.blueprint.geode_robot_cost.0)
    }

    fn has_ore_robot_cost(&self) -> bool {
        self.ore_count >= self.blueprint.ore_robot_cost
    }

    fn has_clay_robot_cost(&self) -> bool {
        self.ore_count >= self.blueprint.clay_robot_cost
    }

    fn has_obsidian_robot_cost(&self) -> bool {
        self.ore_count >= self.blueprint.obsidian_robot_cost.0
            && self.clay_count >= self.blueprint.obsidian_robot_cost.1
    }

    fn has_geode_robot_cost(&self) -> bool {
        self.ore_count >= self.blueprint.geode_robot_cost.0
            && self.obsidian_count >= self.blueprint.geode_robot_cost.1
    }

    fn get_state(
        &self,
        minute: usize,
    ) -> (
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
    ) {
        (
            minute,
            self.ore_robots,
            self.clay_robots,
            self.obsidian_robots,
            self.geode_robots,
            self.ore_count,
            self.clay_count,
            self.obsidian_count,
            self.geode_count,
        )
    }
}

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    id: usize,
    ore_robot_cost: usize,
    clay_robot_cost: usize,
    obsidian_robot_cost: (usize, usize),
    geode_robot_cost: (usize, usize),
}

#[derive(Debug, Clone, PartialEq, Copy, Eq, Hash)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

fn parse_input(input: &str) -> Vec<Factory> {
    input
        .lines()
        .map(|l| {
            let mut words = l.split(" ").into_iter();
            Factory {
                blueprint: Blueprint {
                    id: words
                        .nth(1)
                        .unwrap()
                        .strip_suffix(":")
                        .unwrap()
                        .parse::<usize>()
                        .expect("couldn't parse factory id."),
                    ore_robot_cost: words
                        .nth(4)
                        .unwrap()
                        .parse::<usize>()
                        .expect("couldn't parse ore robot cost"),
                    clay_robot_cost: words
                        .nth(5)
                        .unwrap()
                        .parse::<usize>()
                        .expect("couldn't parse clay robot cost"),
                    obsidian_robot_cost: (
                        words
                            .nth(5)
                            .unwrap()
                            .parse::<usize>()
                            .expect("couldn't parse obsidian robot ore cost"),
                        words
                            .nth(2)
                            .unwrap()
                            .parse::<usize>()
                            .expect("couldn't parse obsidian robot clay cost"),
                    ),
                    geode_robot_cost: (
                        words
                            .nth(5)
                            .unwrap()
                            .parse::<usize>()
                            .expect("couldn't parse geode robot ore cost"),
                        words
                            .nth(2)
                            .unwrap()
                            .parse::<usize>()
                            .expect("couldn't parse geode robot obsidian cost"),
                    ),
                },
                ore_count: 0,
                clay_count: 0,
                obsidian_count: 0,
                geode_count: 0,
                ore_robots: 1,
                clay_robots: 0,
                obsidian_robots: 0,
                geode_robots: 0,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TESTINPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn test_part1() {
        let expected: usize = 33;
        let actual: usize = part1(TESTINPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let expected: usize = 1707;
        let actual: usize = part2(TESTINPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected: usize = 1115;
        let actual: usize = part1(&get_input());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected: usize = 25056;
        let actual: usize = part2(&get_input());
        assert_eq!(actual, expected);
    }
}
