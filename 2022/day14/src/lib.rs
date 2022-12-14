pub fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
    println!("part two: {:?}", part2(&input));
}

#[derive(Clone, Debug, Copy, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Copy, Clone)]
struct Line {
    from: Point,
    to: Point,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn is_on_line(&self, line: &Line) -> bool {
        let dxc = self.x - line.from.x;
        let dyc = self.y - line.from.y;

        let dxl = line.to.x - line.from.x;
        let dyl = line.to.y - line.from.y;

        let cross = dxc * dyl - dyc * dxl;

        if cross != 0 {
            return false;
        }

        if dxl.abs() > dyl.abs() {
            if dxl > 0 {
                line.from.x <= self.x && self.x <= line.to.x
            } else {
                line.to.x <= self.x && self.x <= line.from.x
            }
        } else {
            if dyl > 0 {
                line.from.y <= self.y && self.y <= line.to.y
            } else {
                line.to.y <= self.y && self.y <= line.from.y
            }
        }
    }
}

impl Line {
    fn new(from: Point, to: Point) -> Self {
        Self { from, to }
    }

    fn get_intersection(&self, other: &Line) -> Option<Point> {
        if other.is_horizontal_line() {
            unreachable!("this shouldn't happen!")
        }
        if !self.is_horizontal_line() {
            if other.from.x != self.from.x {
                return None;
            }
            let min_y = self.to.y.min(self.from.y);
            if other.from.y < min_y {
                return Some(Point::new(other.from.x, min_y));
            }
            return None;
        }
        let min_x = self.from.x.min(self.to.x);
        let max_x = self.from.x.max(self.to.x);
        if other.from.x >= min_x && other.from.x <= max_x {
            let min_y = self.to.y.min(self.from.y);
            if other.from.y < min_y {
                return Some(Point::new(other.from.x, min_y));
            }
        }
        return None;
    }

    fn is_horizontal_line(&self) -> bool {
        self.from.y == self.to.y && self.from.x != self.to.x
    }
}

// fn print_cave(cave: &Vec<Line>) {
//     let min_x = cave
//         .iter()
//         .map(|v| if v.from.x < v.to.x { v.from.x } else { v.to.x })
//         .min()
//         .unwrap();
//     let max_x = cave
//         .iter()
//         .map(|v| if v.from.x < v.to.x { v.to.x } else { v.from.x })
//         .max()
//         .unwrap();
//     let min_y = cave
//         .iter()
//         .map(|v| if v.from.y < v.to.y { v.from.y } else { v.to.y })
//         .min()
//         .unwrap();
//     let max_y = cave
//         .iter()
//         .map(|v| if v.from.y < v.to.y { v.to.y } else { v.from.y })
//         .max()
//         .unwrap();
//     for y in min_y..=max_y {
//         for x in min_x..=max_x {
//             if is_occupied(&Point::new(x, y), cave) {
//                 print!["#"]
//             } else {
//                 print!["."]
//             }
//         }
//         print!["\n"]
//     }
// }

pub fn get_input() -> String {
    include_str!("../../inputs/14.txt").replace("\r", "")
}

fn get_collision_point(line: &Line, cave: &Vec<Line>) -> Option<(Line, Point)> {
    cave.iter()
        .filter_map(|l| {
            if let Some(p) = l.get_intersection(line) {
                Some((l.clone(), p))
            } else {
                None
            }
        })
        .min_by(|(_, a), (_, b)| a.y.cmp(&b.y))
}

fn is_occupied(point: &Point, cave: &Vec<Line>) -> bool {
    for vec in cave {
        if point.is_on_line(vec) {
            return true;
        }
    }
    false
}

fn simulate_pouring(pouring_line: &Line, cave: &mut Vec<Line>, bottom: isize) -> Option<Point> {
    // let mut units = 0;
    if let Some((collision_line, collision_point)) = get_collision_point(&pouring_line, &cave) {
        if !is_occupied(&Point::new(collision_point.x - 1, collision_point.y), &cave) {
            let new_pouring_line = Line::new(
                Point::new(collision_point.x - 1, collision_point.y),
                Point::new(collision_point.x - 1, bottom),
            );
            return simulate_pouring(&new_pouring_line, cave, bottom);
        } else if !is_occupied(&Point::new(collision_point.x + 1, collision_point.y), &cave) {
            let new_pouring_line = Line::new(
                Point::new(collision_point.x + 1, collision_point.y),
                Point::new(collision_point.x + 1, bottom),
            );
            return simulate_pouring(&new_pouring_line, cave, bottom);
        } else {
            let landing_point = Point::new(collision_point.x, collision_point.y - 1);
            cave.push(Line::new(landing_point, collision_point));
            return Some(landing_point);
        }
    }
    return None;
}

pub fn part1(input: &str) -> usize {
    let mut units = 0;
    let (mut cave, bottom) = parse_cave(input);
    let pouring_line = Line::new(Point::new(500, 0), Point::new(500, bottom));
    while let Some(_) = simulate_pouring(&pouring_line, &mut cave, bottom) {
        units += 1;
    }
    units
}

pub fn part2(input: &str) -> usize {
    let mut units = 0;
    let (mut cave, bottom) = parse_cave(input);
    let new_bottom = bottom + 2;
    cave.push(Line::new(
        Point::new(0, new_bottom),
        Point::new(10000, new_bottom),
    ));
    let pouring_line = Line::new(Point::new(500, 0), Point::new(500, new_bottom));
    while let Some(p) = simulate_pouring(&pouring_line, &mut cave, bottom) {
        units += 1;
        if p == Point::new(500, 0) {
            break;
        }
    }
    units
}

fn parse_cave(input: &str) -> (Vec<Line>, isize) {
    let mut res = Vec::new();
    let mut bottom = 0;
    for line in input.lines() {
        let mut points = line.split(" -> ");
        let mut from = parse_point(points.next().unwrap());
        for point in points {
            if from.y > bottom {
                bottom = from.y
            }
            let to = parse_point(point);
            res.push(Line::new(from, to.clone()));
            from = to
        }
    }
    (res, bottom)
}

fn parse_point(p: &str) -> Point {
    let (a, b) = p.split_once(",").unwrap();
    Point {
        x: a.parse::<isize>().expect("couldn't parse point x"),
        y: b.parse::<isize>().expect("couldn't parse point b"),
    }
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

    // #[test]
    // fn test_part2_with_input() {
    //     let expected: usize = 27426;
    //     let actual: usize = part2(&get_input());
    //     assert_eq!(actual, expected);
    // }
}
