use std::collections::VecDeque;

pub fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
    println!("part two: {:?}", part2(&input));
}

pub fn get_input() -> String {
    include_str!("../../inputs/11.txt").replace("\r", "")
}

pub fn part1(_input: &str) -> usize {
    let mut monkeys = get_monkeys();
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let mut throws: Vec<(usize, usize)> = Vec::new();
            let m = monkeys.get_mut(i).unwrap();
            while let Some(item) = m.items.pop_front() {
                m.inspects += 1;
                let op = m.op;
                let new = op(item);
                let test = m.test;
                let throw_to = test(new / 3);
                throws.push((throw_to, new / 3))
            }
            for i in throws {
                let m = monkeys.get_mut(i.0).unwrap();
                m.items.push_back(i.1)
            }
        }
    }
    monkeys.sort_by(|a, b| b.inspects.partial_cmp(&a.inspects).unwrap());
    monkeys[0].inspects * monkeys[1].inspects
}

pub fn part2(_input: &str) -> usize {
    let mut monkeys = get_monkeys();
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let mut throws: Vec<(usize, usize)> = Vec::new();
            let m = monkeys.get_mut(i).unwrap();
            while let Some(item) = m.items.pop_front() {
                m.inspects += 1;
                let op = m.op;
                let new = op(item) % 9699690; //chinese remainder theorem - we want a number that is smaller but leaves the same remainder
                let test = m.test;
                let throw_to = test(new);
                throws.push((throw_to, new))
            }
            for i in throws {
                let m = monkeys.get_mut(i.0).unwrap();
                m.items.push_back(i.1)
            }
        }
    }
    monkeys.sort_by(|a, b| b.inspects.partial_cmp(&a.inspects).unwrap());
    monkeys[0].inspects * monkeys[1].inspects
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    op: fn(usize) -> usize,
    test: fn(usize) -> usize,
    inspects: usize,
}

fn get_monkeys() -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    monkeys.push(Monkey {
        items: VecDeque::from([91, 66]),
        op: |item| item * 13,
        test: |item| {
            if item % 19 == 0 {
                6
            } else {
                2
            }
        },
        inspects: 0,
    });
    monkeys.push(Monkey {
        items: VecDeque::from([78, 97, 59]),
        op: |item| item + 7,
        test: |item| {
            if item % 5 == 0 {
                0
            } else {
                3
            }
        },
        inspects: 0,
    });
    monkeys.push(Monkey {
        items: VecDeque::from([57, 59, 97, 84, 72, 83, 56, 76]),
        op: |item| item + 6,
        test: |item| {
            if item % 11 == 0 {
                5
            } else {
                7
            }
        },
        inspects: 0,
    });
    monkeys.push(Monkey {
        items: VecDeque::from([81, 78, 70, 58, 84]),
        op: |item| item + 5,
        test: |item| {
            if item % 17 == 0 {
                6
            } else {
                0
            }
        },
        inspects: 0,
    });
    monkeys.push(Monkey {
        items: VecDeque::from([60]),
        op: |item| item + 8,
        test: |item| {
            if item % 7 == 0 {
                1
            } else {
                3
            }
        },
        inspects: 0,
    });
    monkeys.push(Monkey {
        items: VecDeque::from([57, 69, 63, 75, 62, 77, 72]),
        op: |item| item * 5,
        test: |item| {
            if item % 13 == 0 {
                7
            } else {
                4
            }
        },
        inspects: 0,
    });
    monkeys.push(Monkey {
        items: VecDeque::from([73, 66, 86, 79, 98, 87]),
        op: |item| item * item,
        test: |item| {
            if item % 3 == 0 {
                5
            } else {
                2
            }
        },
        inspects: 0,
    });
    monkeys.push(Monkey {
        items: VecDeque::from([95, 89, 63, 67]),
        op: |item| item + 2,
        test: |item| {
            if item % 2 == 0 {
                1
            } else {
                4
            }
        },
        inspects: 0,
    });
    monkeys
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TESTINPUT_1: &str = "";

    // #[test]
    // fn test_part1() {
    //     let expected = 0;
    //     let actual = part1(TESTINPUT_1);
    //     assert_eq!(actual, expected);
    // }

    // #[test]
    // fn test_part2() {
    //     let expected = 0;
    //     let actual = part2(TESTINPUT_1);
    //     assert_eq!(actual, expected);
    // }

    #[test]
    fn test_part1_with_input() {
        let expected = 101436;
        let actual = part1(&get_input());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected = 19754471646;
        let actual = part2(&get_input());
        assert_eq!(actual, expected);
    }
}
