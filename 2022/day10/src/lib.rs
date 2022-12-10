pub fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
    println!("part two: \n{}", part2(&input));
}

pub fn get_input() -> String {
    include_str!("../../inputs/10.txt").replace("\r", "")
}

pub fn part1(input: &str) -> isize {
    let mut signal_sum = 0isize;
    let mut cpu = CPU::new(get_instructions(input));
    while cpu.has_instructions() {
        cpu.do_cycle();
        match cpu.cycle + 1 {
            20 | 60 | 100 | 140 | 180 | 220 => {
                signal_sum += (cpu.cycle + 1) as isize * cpu.register
            }
            _ => {}
        }
    }
    signal_sum
}

pub fn part2(input: &str) -> String {
    let mut crt = String::new();
    let mut cpu = CPU::new(get_instructions(input));
    while cpu.has_instructions() {
        cpu.draw_pixel(&mut crt);
        cpu.do_cycle();
    }
    crt
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Add(isize),
}

struct CPU {
    cycle: usize,
    instructions: Vec<Instruction>,
    next_instruction_at: usize,
    current_instruction: usize,
    register: isize,
}

impl CPU {
    fn new(instructions: Vec<Instruction>) -> Self {
        let next_instruction_at = match instructions[0] {
            Instruction::Noop => 1usize,
            Instruction::Add(_) => 2usize,
        };
        Self {
            cycle: 0,
            instructions,
            next_instruction_at,
            current_instruction: 0,
            register: 1,
        }
    }

    fn do_cycle(&mut self) {
        self.cycle += 1;
        if self.next_instruction_at == self.cycle {
            match self.instructions[self.current_instruction] {
                Instruction::Noop => {
                    self.current_instruction += 1;
                }
                Instruction::Add(num) => {
                    self.register += num;
                    self.current_instruction += 1;
                }
            }
            self.set_next_instruction_at();
        }
    }

    fn draw_pixel(&self, crt: &mut String) {
        if self.cycle > 1 && self.cycle % 40 == 0 {
            crt.push('\n');
        }
        let current_pixel = self.cycle % 40;
        match (self.register - current_pixel as isize).abs() {
            n if n < 2 => crt.push('#'),
            _ => crt.push('.'),
        }
    }

    fn set_next_instruction_at(&mut self) {
        if self.has_instructions() {
            match self.instructions[self.current_instruction] {
                Instruction::Noop => self.next_instruction_at = self.cycle + 1,
                Instruction::Add(_) => self.next_instruction_at = self.cycle + 2,
            }
        }
    }

    fn has_instructions(&self) -> bool {
        self.current_instruction < self.instructions.len()
    }
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
