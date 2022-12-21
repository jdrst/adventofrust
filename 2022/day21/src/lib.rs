use std::collections::HashMap;

pub fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
    println!("part two: {:?}", part2(&input));
}

pub fn get_input() -> String {
    include_str!("../../inputs/21.txt").replace("\r", "")
}

pub fn part1(input: &str) -> isize {
    let monkeys = parse_input(input);
    monkeys.get("root").unwrap().do_job(&monkeys)
}
pub fn part2(input: &str) -> isize {
    let monkeys = parse_input(input);

    let (path, equal) = Monkey::get_yell_path("root".to_string(), &monkeys);

    Monkey::get_hmn_yell_value(equal, path, &monkeys)
}

fn parse_input(input: &str) -> HashMap<String, Monkey> {
    let mut monkeys: HashMap<String, Monkey> = HashMap::new();
    input.lines().for_each(|l| {
        let (name, job) = l.split_once(": ").unwrap();
        let monkey = if let Ok(number) = job.parse::<isize>() {
            Monkey {
                job: Job::Yell(number),
            }
        } else {
            let operation = job.split(" ").collect::<Vec<_>>();
            match operation[1] {
                "+" => Monkey {
                    job: Job::Add(operation[0].into(), operation[2].into()),
                },
                "-" => Monkey {
                    job: Job::Subtract(operation[0].into(), operation[2].into()),
                },
                "*" => Monkey {
                    job: Job::Multiply(operation[0].into(), operation[2].into()),
                },
                "/" => Monkey {
                    job: Job::Divide(operation[0].into(), operation[2].into()),
                },
                _ => unreachable!("this shouldn't happen."),
            }
        };
        monkeys.insert(name.into(), monkey);
    });
    monkeys
}

struct Monkey {
    job: Job,
}

impl Monkey {
    fn do_job(&self, monkeys: &HashMap<String, Monkey>) -> isize {
        match &self.job {
            Job::Yell(i) => *i,
            Job::Add(a, b) => {
                monkeys.get(a.as_str()).unwrap().do_job(monkeys)
                    + monkeys.get(b.as_str()).unwrap().do_job(monkeys)
            }
            Job::Subtract(a, b) => {
                monkeys.get(a.as_str()).unwrap().do_job(monkeys)
                    - monkeys.get(b.as_str()).unwrap().do_job(monkeys)
            }
            Job::Multiply(a, b) => {
                monkeys.get(a.as_str()).unwrap().do_job(monkeys)
                    * monkeys.get(b.as_str()).unwrap().do_job(monkeys)
            }
            Job::Divide(a, b) => {
                monkeys.get(a.as_str()).unwrap().do_job(monkeys)
                    / monkeys.get(b.as_str()).unwrap().do_job(monkeys)
            }
        }
    }

    fn has_hmn_input(&self, monkeys: &HashMap<String, Monkey>) -> bool {
        match &self.job {
            Job::Yell(_) => false,
            Job::Add(a, b) | Job::Subtract(a, b) | Job::Multiply(a, b) | Job::Divide(a, b) => {
                if a == "humn" || b == "humn" {
                    return true;
                }
                monkeys.get(a.as_str()).unwrap().has_hmn_input(monkeys)
                    | monkeys.get(b.as_str()).unwrap().has_hmn_input(monkeys)
            }
        }
    }

    fn get_hmn_yell_value(
        mut value_before: isize,
        yell_path: Vec<String>,
        monkeys: &HashMap<String, Monkey>,
    ) -> isize {
        let mut path = yell_path.iter();
        let mut next = path.next().unwrap();
        loop {
            let Some(after) = path.next() else { break; };
            let monkey = monkeys.get(next).unwrap();
            match &monkey.job {
                Job::Yell(_) => unreachable!(),
                Job::Add(a, b) => {
                    if after == a {
                        value_before = value_before - monkeys.get(b).unwrap().do_job(monkeys);
                    } else {
                        value_before = value_before - monkeys.get(a).unwrap().do_job(monkeys);
                    }
                }
                Job::Subtract(a, b) => {
                    if after == a {
                        value_before = value_before + monkeys.get(b).unwrap().do_job(monkeys);
                    } else {
                        value_before = value_before - monkeys.get(a).unwrap().do_job(monkeys);
                        value_before = value_before * -1;
                    }
                }
                Job::Multiply(a, b) => {
                    if after == a {
                        value_before = value_before / monkeys.get(b).unwrap().do_job(monkeys);
                    } else {
                        value_before = value_before / monkeys.get(a).unwrap().do_job(monkeys);
                    }
                }
                Job::Divide(a, b) => {
                    if after == a {
                        value_before = value_before * monkeys.get(b).unwrap().do_job(monkeys);
                    } else {
                        value_before = value_before * monkeys.get(a).unwrap().do_job(monkeys);
                    }
                }
            }
            next = after;
        }
        value_before
    }
    fn get_yell_path(mut node: String, monkeys: &HashMap<String, Monkey>) -> (Vec<String>, isize) {
        let mut path = Vec::new();
        let mut equals = 0;
        loop {
            let current_monkey = monkeys.get(node.as_str()).expect("invalid node.");
            if node == "root" {}
            match &current_monkey.job {
                Job::Yell(_) => unreachable!("can't get yell path"),
                Job::Add(a, b) | Job::Subtract(a, b) | Job::Multiply(a, b) | Job::Divide(a, b) => {
                    if a == "humn" || b == "humn" {
                        break;
                    }
                    if monkeys.get(a.as_str()).unwrap().has_hmn_input(monkeys) {
                        path.push(a.to_string());
                        if node == "root" {
                            equals = monkeys.get(b.as_str()).unwrap().do_job(monkeys);
                        }
                        node = a.to_string();
                    } else {
                        path.push(b.to_string());
                        if node == "root" {
                            equals = monkeys.get(a.as_str()).unwrap().do_job(monkeys);
                        }
                        node = b.to_string();
                    }
                }
            }
        }
        path.push("humn".to_string());
        (path, equals)
    }
}

enum Job {
    Yell(isize),
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TESTINPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_part1() {
        let expected = 152;
        let actual = part1(TESTINPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let expected = 301;
        let actual = part2(TESTINPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected = 256997859093114;
        let actual = part1(&get_input());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected = 3952288690726;
        let actual = part2(&get_input());
        assert_eq!(actual, expected);
    }
}
