use std::collections::{BTreeMap, HashMap, HashSet};

pub fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
    println!("part two: {:?}", part2(&input));
}

pub fn get_input() -> String {
    include_str!("../../inputs/16.txt").replace("\r", "")
}

pub fn part1(input: &str) -> usize {
    let (entry, network) = parse_input(input);

    move_through_cave(&entry, 30, network)
}

pub fn part2(input: &str) -> usize {
    let (entry, mut network) = parse_input(input);
    let nw = network
        .iter_mut()
        .fold(BTreeMap::new(), |mut map, (key, value)| {
            map.insert(key.clone(), value.clone());
            map
        });

    move_through_cave_part2(&entry, &entry, 26, 26, nw, &mut 0, &mut HashSet::new())
}

fn maximum_possible(
    current_room: &str,
    elephant_current_room: &str,
    time_left: usize,
    elephant_time_left: usize,
    network: &BTreeMap<String, Room>,
) -> usize {
    let mut max = sum_resbt(network);
    let my_rooms = &network.get(current_room).unwrap().paths;
    let elephant_rooms = &network.get(elephant_current_room).unwrap().paths;
    my_rooms
        .iter()
        .zip(elephant_rooms.iter())
        .map(|(me, el)| {
            (
                (network.get(&me.0).unwrap(), me.1),
                (network.get(&el.0).unwrap(), el.1),
            )
        })
        .for_each(|(me, el)| {
            max += if time_left > me.1 && !me.0.open {
                me.0.flow_rate * (time_left - me.1)
            } else {
                0
            } + if elephant_time_left > el.1 && !el.0.open {
                el.0.flow_rate * (elephant_time_left - el.1)
            } else {
                0
            }
        });
    max
}

fn move_through_cave_part2(
    my_current_room: &str,
    elephant_current_room: &str,
    mut time_left: usize,
    mut elephant_time_left: usize,
    mut network: BTreeMap<String, Room>,
    current_best: &mut usize,
    visited: &mut HashSet<Vec<Room>>,
) -> usize {
    if time_left < 2 && elephant_time_left < 2 {
        let sum = sum_resbt(&network);
        if sum > *current_best {
            println!["found new best: {sum}"];
            *current_best = sum
        }
        return sum;
    }

    let mut res = 0;

    if let Some(mut room) = network.get_mut(my_current_room) {
        if room.flow_rate > 0 && !room.open && time_left > 0 {
            time_left -= 1;
            room.open = true;
            room.opened_at = time_left;
        }
    }

    if let Some(mut room) = network.get_mut(elephant_current_room) {
        if room.flow_rate > 0 && !room.open && elephant_time_left > 0 {
            elephant_time_left -= 1;
            room.open = true;
            room.opened_at = elephant_time_left;
        }
    }

    if maximum_possible(
        my_current_room,
        elephant_current_room,
        time_left,
        elephant_time_left,
        &network,
    ) <= *current_best
    {
        return 0;
    };

    let vec = network.iter().map(|(_, v)| v).cloned().collect();
    if visited.contains(&vec) {
        return 0;
    } else {
        visited.insert(vec);
    }

    let reachable = network
        .get(my_current_room)
        .unwrap()
        .paths
        .iter()
        .filter(|p| {
            let r = network.get(&p.0).unwrap();
            !r.open && r.flow_rate > 0 && p.1 + 2 < time_left
        })
        .collect::<Vec<_>>();

    let elephant_reachable = network
        .get(elephant_current_room)
        .unwrap()
        .paths
        .iter()
        .filter(|p| {
            let r = network.get(&p.0).unwrap();
            !r.open && r.flow_rate > 0 && p.1 + 2 < elephant_time_left
        })
        .collect::<Vec<_>>();

    if reachable.len() == 0 && elephant_reachable.len() == 0 {
        let sum = sum_resbt(&network);
        if sum > *current_best {
            *current_best = sum
        }
        return sum;
    }

    for path in &reachable {
        if elephant_reachable.len() > 0 {
            for e_path in &elephant_reachable {
                if path.0 != e_path.0 {
                    res = res.max(move_through_cave_part2(
                        &path.0,
                        &e_path.0,
                        time_left - path.1,
                        elephant_time_left - e_path.1,
                        network.clone(),
                        current_best,
                        visited,
                    ))
                }
            }
        } else {
            res = res.max(move_through_cave_part2(
                &path.0,
                elephant_current_room,
                time_left - path.1,
                elephant_time_left,
                network.clone(),
                current_best,
                visited,
            ))
        }
    }
    for e_path in elephant_reachable {
        if reachable.len() > 0 {
            for path in &reachable {
                if path.0 != e_path.0 {
                    res = res.max(move_through_cave_part2(
                        &path.0,
                        &e_path.0,
                        time_left - path.1,
                        elephant_time_left - e_path.1,
                        network.clone(),
                        current_best,
                        visited,
                    ))
                    // }
                }
            }
        } else {
            res = res.max(move_through_cave_part2(
                my_current_room,
                &e_path.0,
                time_left,
                elephant_time_left - e_path.1,
                network.clone(),
                current_best,
                visited,
            ))
        }
    }
    res
}

fn move_through_cave(
    current_room: &str,
    mut time_left: usize,
    mut network: HashMap<String, Room>,
) -> usize {
    if time_left < 2 {
        let res = sum_res(&network);
        return res;
    }
    if let Some(mut room) = network.get_mut(current_room) {
        if room.flow_rate > 0 && !room.open {
            time_left -= 1;
            room.open = true;
            room.opened_at = time_left;
        }
    }

    let paths = network.get(current_room).unwrap().paths.clone();
    let reachable = paths
        .iter()
        .filter(|p| !network.get(&p.0).unwrap().open && p.1 < time_left)
        .count();
    if reachable == 0 {
        return sum_res(&network);
    }
    paths
        .iter()
        .filter(|p| {
            !network.get(&p.0).unwrap().open
                && network.get(&p.0).unwrap().flow_rate > 0
                && p.1 < time_left
        })
        .map(|p| move_through_cave(&p.0, time_left - p.1, network.clone()))
        .max()
        .unwrap_or(0)
}

fn sum_res(network: &HashMap<String, Room>) -> usize {
    let mut res = 0;
    for (_, room) in network.iter() {
        if room.open {
            res += room.opened_at * room.flow_rate;
        }
    }
    res
}

fn sum_resbt(network: &BTreeMap<String, Room>) -> usize {
    let mut res = 0;
    for (_, room) in network.iter() {
        if room.open {
            res += room.opened_at * room.flow_rate;
        }
    }
    res
}

#[derive(Debug, Clone)]
struct Valve<'a> {
    // name: String,
    flow_rate: usize,
    tunnels: Vec<&'a str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Room {
    // name: String,
    open: bool,
    flow_rate: usize,
    opened_at: usize,
    paths: Vec<(String, usize)>,
}

fn parse_input(input: &str) -> (String, HashMap<String, Room>) {
    let mut intermediate = HashMap::new();

    let entry = "AA";
    input.lines().for_each(|l| {
        let (name, rest) = l
            .strip_prefix("Valve ")
            .expect("line not starting with 'Valve '")
            .split_once(" ")
            .unwrap();
        let (flow_rate_str, rest) = rest
            .strip_prefix("has flow rate=")
            .expect("line is not continuing with 'has flow rate='")
            .split_once(";")
            .unwrap();
        let flow_rate = flow_rate_str
            .parse::<usize>()
            .expect("can't parse flow rate");
        let tunnels = rest
            .splitn(6, " ")
            .last()
            .expect("can't split until valves")
            .split(",")
            .map(|s| s.trim())
            .collect();
        intermediate.insert(name, Valve { flow_rate, tunnels });
    });

    let rooms_with_valves = intermediate
        .iter()
        .filter(|(_, v)| v.flow_rate > 0)
        .map(|(name, _)| name.to_string())
        .collect::<Vec<String>>();

    let mut res = rooms_with_valves
        .iter()
        .map(|name| {
            let paths =
                get_shortest_paths_to_valves_with_cost(name, &intermediate, &rooms_with_valves);
            (
                name.to_string(),
                Room {
                    open: false,
                    flow_rate: intermediate.get(name.as_str()).unwrap().flow_rate,
                    opened_at: 0,
                    paths,
                },
            )
        })
        .collect::<HashMap<String, Room>>();
    if !rooms_with_valves.contains(&entry.to_string()) {
        res.insert(
            entry.to_string(),
            Room {
                open: false,
                flow_rate: 0,
                opened_at: 0,
                paths: get_shortest_paths_to_valves_with_cost(
                    entry,
                    &intermediate,
                    &rooms_with_valves,
                ),
            },
        );
    }
    (entry.to_owned(), res)
}
fn get_shortest_paths_to_valves_with_cost(
    current_room: &str,
    network: &HashMap<&str, Valve>,
    rooms_with_valves: &Vec<String>,
) -> Vec<(String, usize)> {
    rooms_with_valves
        .iter()
        .filter(|r| *r != current_room)
        .map(|r| {
            (
                r.to_string(),
                get_shortest_path_to(current_room, r, network, Vec::new(), 0),
            )
        })
        .collect()
}

fn get_shortest_path_to(
    current_room: &str,
    target_room: &str,
    network: &HashMap<&str, Valve>,
    mut from: Vec<String>,
    mut cost: usize,
) -> usize {
    from.push(current_room.to_string());
    let next_rooms = &network.get(&current_room).unwrap().tunnels;
    cost += 1;
    for room_str in next_rooms {
        if *room_str == target_room {
            return cost;
        }
    }
    if let Some(min) = next_rooms
        .iter()
        .filter(|nr| !from.contains(&nr.to_string()))
        .map(|nr| get_shortest_path_to(nr, target_room, network, from.clone(), cost))
        .min()
    {
        return min;
    }
    usize::MAX
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TESTINPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_part1() {
        let expected: usize = 1651;
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
        let expected: usize = 1796;
        let actual: usize = part1(&get_input());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected: usize = 1999;
        let actual: usize = part2(&get_input());
        assert_eq!(actual, expected);
    }
}
