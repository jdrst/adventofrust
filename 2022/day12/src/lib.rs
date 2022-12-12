use std::collections::{BinaryHeap, HashMap, HashSet};

pub fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
    println!("part two: {:?}", part2(&input));
}

pub fn get_input() -> String {
    include_str!("../../inputs/12.txt").replace("\r", "")
}

pub fn part1(input: &str) -> usize {
    let (grid, (start, end)) = make_grid(input);

    a_star(start, end, &grid).unwrap_or(0)
}

pub fn part2(input: &str) -> usize {
    let (grid, (_, end)) = make_grid(input);

    bfs(end, 0, &grid).unwrap()
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone, Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: Coordinate,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn possible_steps(&self, grid: &Vec<Vec<u8>>) -> Vec<Coordinate> {
        self.neighbours_in_bounds(grid)
            .into_iter()
            .filter(|n| (self.is_allowed_move(n, grid)))
            .collect()
    }

    fn neighbours_in_bounds(&self, grid: &Vec<Vec<u8>>) -> Vec<Self> {
        let mut neighbours = Vec::with_capacity(4);
        if self.y < grid.len() - 1 {
            neighbours.push(Coordinate::new(self.x, self.y + 1));
        }
        if self.x < grid[self.y].len() - 1 {
            neighbours.push(Coordinate::new(self.x + 1, self.y));
        }
        if self.x > 0 {
            neighbours.push(Coordinate::new(self.x - 1, self.y))
        }
        if self.y > 0 {
            neighbours.push(Coordinate::new(self.x, self.y - 1))
        }
        neighbours
    }

    fn is_allowed_move(&self, to: &Coordinate, grid: &Vec<Vec<u8>>) -> bool {
        self.get_height(grid) + 1 >= to.get_height(grid)
    }

    fn get_height(&self, grid: &Vec<Vec<u8>>) -> u8 {
        grid[self.y][self.x]
    }
}

fn a_star(start: Coordinate, end: Coordinate, grid: &Vec<Vec<u8>>) -> Option<usize> {
    let mut heap = BinaryHeap::<State>::new();
    let mut cost_to = HashMap::<Coordinate, usize>::new();
    cost_to.insert(start, 0);

    heap.push(State {
        cost: manhattan(start, end),
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == end {
            return Some(cost);
        }

        for neighbour in position.possible_steps(grid) {
            let cost_to_neighbour = cost_to.get(&position).unwrap_or(&usize::MAX) + 1; //distance is always 1
            let best_known_cost_to_neighbour = cost_to.get(&neighbour).unwrap_or(&usize::MAX);

            if cost_to_neighbour < *best_known_cost_to_neighbour {
                cost_to.insert(neighbour, cost_to_neighbour);
                heap.push(State {
                    cost: cost_to_neighbour + manhattan(neighbour, end),
                    position: neighbour,
                });
            }
        }
    }
    None
}

fn bfs(start: Coordinate, target_height: u8, grid: &Vec<Vec<u8>>) -> Option<usize> {
    let mut heap = BinaryHeap::<State>::new();
    let mut visited = HashSet::<Coordinate>::new();

    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        let pos_height = position.get_height(grid);
        if pos_height == target_height {
            return Some(cost);
        };
        for next in position.neighbours_in_bounds(grid) {
            if next.get_height(grid) + 1 >= pos_height && !visited.contains(&next) {
                visited.insert(next);
                heap.push(State {
                    cost: cost + 1,
                    position: next,
                });
            }
        }
    }
    None
}

fn manhattan(start: Coordinate, end: Coordinate) -> usize {
    start.x.abs_diff(end.x) + start.y.abs_diff(end.y)
}

fn make_grid(input: &str) -> (Vec<Vec<u8>>, (Coordinate, Coordinate)) {
    let width = input.lines().next().unwrap().len();
    let mut start = Coordinate::new(0, 0);
    let mut end = Coordinate::new(0, 0);
    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(width);
    for (y, line) in input.lines().enumerate() {
        grid.push(
            line.chars()
                .enumerate()
                .map(|(x, c)| match c as u8 {
                    b'S' => {
                        start = Coordinate::new(x, y);
                        b'a' - b'a'
                    }
                    b'E' => {
                        end = Coordinate::new(x, y);
                        b'z' - b'a'
                    }
                    _ => c as u8 - b'a',
                })
                .collect(),
        );
    }
    (grid, (start, end))
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TESTINPUT_1: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part1() {
        let expected = 31;
        let actual = part1(TESTINPUT_1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let expected = 29;
        let actual = part2(TESTINPUT_1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected = 391;
        let actual = part1(&get_input());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected = 386;
        let actual = part2(&get_input());
        assert_eq!(actual, expected);
    }
}
