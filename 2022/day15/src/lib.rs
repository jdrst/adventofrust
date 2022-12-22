use std::collections::HashSet;

pub fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input, 2_000_000));
    println!("part two: {:?}", part2(&input, 4_000_000));
}

pub fn get_input() -> String {
    include_str!("../../inputs/15.txt").replace("\r", "")
}

pub fn part1(input: &str, y_pos: isize) -> isize {
    let sensors = parse_input(input);
    let mut ranges = Vec::new();
    let mut beacons_in_line = HashSet::new();
    let mut leftmost_x = 0;
    let mut rightmost_x = 0;
    for sensor in sensors.iter() {
        let min_x = sensor.left_uncovered_x(y_pos) + 1;
        let max_x = sensor.right_uncovered_x_for(y_pos) - 1;
        leftmost_x = leftmost_x.min(min_x);
        rightmost_x = rightmost_x.max(max_x);
        if min_x < max_x {
            ranges.push(min_x..=max_x);
            if sensor.closest_beacon.y == y_pos {
                beacons_in_line.insert((sensor.closest_beacon.x, sensor.closest_beacon.y));
            }
        }
    }
    let mut sum = 0;
    for x in leftmost_x..=rightmost_x {
        if ranges.iter().any(|r| r.contains(&x)) {
            sum += 1;
        }
    }
    sum - beacons_in_line.len() as isize
}

pub fn part2(input: &str, upper_bound: isize) -> isize {
    let sensors = parse_input(input);
    for y in 0..=upper_bound {
        let mut x = 0;
        'foo: while x <= upper_bound {
            // println!["searching for ({x},{y})"];
            if let Some(sensor) = sensors.iter().find(|s| s.is_in_range((x, y))) {
                x = sensor.right_uncovered_x_for(y);
                continue 'foo;
            } else {
                return x * 4_000_000 + y;
            }
        }
    }
    0
}

#[derive(Debug)]
struct Beacon {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Sensor {
    x: isize,
    y: isize,
    closest_beacon: Beacon,
}

impl Sensor {
    fn right_uncovered_x_for(&self, y: isize) -> isize {
        let distance = self.manhattan_to_beacon() as isize;
        self.x + (distance - self.y.abs_diff(y) as isize) + 1
    }

    fn left_uncovered_x(&self, y: isize) -> isize {
        let distance = self.manhattan_to_beacon() as isize;
        self.x - (distance - self.y.abs_diff(y) as isize) - 1
        // x + distance as isize
    }

    fn is_in_range(&self, (x, y): (isize, isize)) -> bool {
        Self::manhattan((self.x, self.y), (x, y)) <= self.manhattan_to_beacon()
    }

    fn manhattan((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> usize {
        x1.abs_diff(x2) + y1.abs_diff(y2)
    }

    fn manhattan_to_beacon(&self) -> usize {
        Self::manhattan(
            (self.x, self.y),
            (self.closest_beacon.x, self.closest_beacon.y),
        )
    }
}

fn parse_input(input: &str) -> Vec<Sensor> {
    let mut sensors = Vec::new();
    for line in input.lines() {
        let (sensor_str, beacon_str) = line.split_once(":").expect("no a sensor.");

        let (sensor_x_str, sensor_y_str) = sensor_str
            .trim_start_matches("Sensor at x=")
            .split_once(",")
            .expect("can't split sensor coordinates");
        let sensor_x = sensor_x_str
            .parse::<isize>()
            .expect("can't parse sensor x.");
        let sensor_y = sensor_y_str
            .trim_start_matches(" y=")
            .parse::<isize>()
            .expect("can't parse sensor y.");

        let (beacon_x_str, beacon_y_str) = beacon_str
            .trim_start_matches(" closest beacon is at x=")
            .split_once(",")
            .expect("can't split beacon coordinates");
        let beacon_x = beacon_x_str
            .parse::<isize>()
            .expect("can't parse beacon x.");
        let beacon_y = beacon_y_str
            .trim_start_matches(" y=")
            .parse::<isize>()
            .expect("can't parse beacon y.");
        sensors.push(Sensor {
            x: sensor_x,
            y: sensor_y,
            closest_beacon: Beacon {
                x: beacon_x,
                y: beacon_y,
            },
        })
    }
    sensors
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TESTINPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_part1() {
        let expected: isize = 26;
        let actual: isize = part1(TESTINPUT, 10);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let expected: isize = 56_000_011;
        let actual: isize = part2(TESTINPUT, 20);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected: isize = 4665948;
        let actual: isize = part1(&get_input(), 2_000_000);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected: isize = 13543690671045;
        let actual: isize = part2(&get_input(), 4_000_000);
        assert_eq!(actual, expected);
    }
}
