use std::{iter::Peekable, str::Lines};

pub fn main() {
    let input = get_input();
    println!("part one: {:?}", part1(&input));
    println!("part two: {:?}", part2(&input));
}

pub fn get_input() -> String {
    include_str!("../../inputs/07.txt").replace("\r", "")
}

pub fn part1(input: &str) -> usize {
    let (_, inp) = input.split_once("\n").unwrap();
    let mut top_level = Vec::new();
    let mut sub_levels = Vec::new();
    walk_dir(&mut inp.lines().peekable(), &mut top_level, &mut sub_levels);
    sub_levels.iter().filter(|s| **s <= 100000).sum::<usize>()
        + top_level.iter().filter(|s| **s <= 100000).sum::<usize>()
}

pub fn walk_dir(
    lines: &mut Peekable<Lines>,
    current_sizes: &mut Vec<usize>,
    all_dirs: &mut Vec<usize>,
) -> usize {
    if let Some(curr) = lines.next() {
        if curr.starts_with("$") {
            match curr.trim() {
                "$ cd .." => {
                    let current_dir_size =
                        current_sizes.pop().expect("there is no current dir size");
                    all_dirs.push(current_dir_size);
                    *current_sizes
                        .last_mut()
                        .expect("there is no last current size 1") += current_dir_size;
                    return walk_dir(lines, current_sizes, all_dirs);
                }
                "$ ls" => {
                    current_sizes.push(0);
                    return walk_dir(lines, current_sizes, all_dirs);
                }
                _ => {
                    return walk_dir(lines, current_sizes, all_dirs);
                }
            }
        }
        if curr.starts_with("dir") {
            return walk_dir(lines, current_sizes, all_dirs);
        }
        let (size, _) = curr.split_once(" ").expect("not a file entry");
        *current_sizes
            .last_mut()
            .expect("there is no last current size") +=
            size.parse::<usize>().expect("not a filesize");
        return walk_dir(lines, current_sizes, all_dirs);
    }
    0
}

pub fn part2(input: &str) -> usize {
    let (_, inp) = input.split_once("\n").unwrap();
    let mut top_level = Vec::new();
    let mut sub_levels = Vec::new();
    walk_dir(&mut inp.lines().peekable(), &mut top_level, &mut sub_levels);
    let used_space: usize = top_level.iter().sum();
    let free_space_needed = 30000000 - (70000000 - used_space);
    top_level.append(&mut sub_levels);
    *top_level[1..] //we don't need root
        .iter()
        .filter(|s| **s >= free_space_needed)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TESTINPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_part1() {
        let expected = 95437;
        let actual = part1(TESTINPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let expected = 24933642;
        let actual = part2(TESTINPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected = 1243729;
        let actual = part1(&get_input());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_with_input() {
        let expected = 4443914;
        let actual = part2(&get_input());
        assert_eq!(actual, expected);
    }
}
