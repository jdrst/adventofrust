pub fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
    println!("part two: \n{}", part2(&input));
}

pub fn get_input() -> String {
    include_str!("../../inputs/10.txt").replace("\r", "")
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Add(isize),
}

pub fn part1(input: &str) -> isize {
    let mut cycle = 0usize;
    let instructions = get_instructions(input);
    let mut inst_iter = instructions.iter().skip(1).peekable();
    let mut current_instruction = &instructions[0];
    let mut instruction_done_at = match instructions[0] {
        Instruction::Noop => 1usize,
        Instruction::Add(_) => 2usize,
    };
    let mut register = 1isize;
    let mut signal_sum = 0isize;
    loop {
        if let None = inst_iter.peek() {
            break;
        }
        cycle += 1;
        match cycle {
            20 | 60 | 100 | 140 | 180 | 220 => signal_sum += cycle as isize * register,
            _ => {}
        }
        if instruction_done_at == cycle {
            match current_instruction {
                Instruction::Add(num) => {
                    register += num;
                    current_instruction = inst_iter.next().unwrap();
                    match current_instruction {
                        Instruction::Noop => instruction_done_at = cycle + 1,
                        Instruction::Add(_) => instruction_done_at = cycle + 2,
                    }
                }
                Instruction::Noop => {
                    current_instruction = inst_iter.next().unwrap();
                    match current_instruction {
                        Instruction::Noop => instruction_done_at = cycle + 1,
                        Instruction::Add(_) => instruction_done_at = cycle + 2,
                    }
                }
            }
        } else {
        }
    }
    signal_sum
}

pub fn part2(input: &str) -> String {
    let mut crt = String::new();
    let mut cycle = 0usize;
    let instructions = get_instructions(input);
    let mut inst_iter = instructions.iter().skip(1).peekable();
    let mut current_instruction = &instructions[0];
    let mut instruction_done_at = match instructions[0] {
        Instruction::Noop => 1usize,
        Instruction::Add(_) => 2usize,
    };
    let mut register = 1isize;
    loop {
        cycle += 1;
        if cycle > 1 && (cycle - 1) % 40 == 0 {
            crt += "\n";
        }
        let current_pixel = (cycle - 1) % 40;
        if register - 1 == current_pixel as isize
            || register == current_pixel as isize
            || register + 1 == current_pixel as isize
        {
            crt += "#"
        } else {
            crt += "."
        }

        if let None = inst_iter.peek() {
            break;
        }

        if instruction_done_at == cycle {
            match current_instruction {
                Instruction::Add(num) => {
                    register += num;
                    current_instruction = inst_iter.next().unwrap();
                    match current_instruction {
                        Instruction::Noop => instruction_done_at = cycle + 1,
                        Instruction::Add(_) => instruction_done_at = cycle + 2,
                    }
                }
                Instruction::Noop => {
                    current_instruction = inst_iter.next().unwrap();
                    match current_instruction {
                        Instruction::Noop => instruction_done_at = cycle + 1,
                        Instruction::Add(_) => instruction_done_at = cycle + 2,
                    }
                }
            }
        } else {
        }
    }
    crt
}

fn get_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| match l {
            "noop" => Instruction::Noop,
            _ => {
                let (_, num) = l.split_once(" ").unwrap();
                Instruction::Add(num.parse::<isize>().unwrap())
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TESTINPUT_0: &str = "noop
addx 3
addx -5";

    const TESTINPUT_1: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_part1_0() {
        let expected = 0;
        let actual = part1(TESTINPUT_0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1() {
        let expected = 13140;
        let actual = part1(TESTINPUT_1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        let actual = part2(TESTINPUT_1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected = 15680;
        let actual = part1(&get_input());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected = "####.####.###..####.#..#..##..#..#.###..
...#.#....#..#.#....#..#.#..#.#..#.#..#.
..#..###..###..###..####.#....#..#.#..#.
.#...#....#..#.#....#..#.#.##.#..#.###..
#....#....#..#.#....#..#.#..#.#..#.#....
####.#....###..#....#..#..###..##..#....";
        let actual = part2(&get_input());
        assert_eq!(actual, expected);
    }
}
