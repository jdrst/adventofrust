fn main() {
    let input = include_str!["../../inputs/05.txt"];
    println!["part1: {:?}", part1(input)];
}

fn part1(s: &str) -> usize {
    //regex: ^(?=.*([a-z])\1.*)(?=^(?:.*[aeiou].*){3,}?$)(?:(?!xy).)*$
    let lines = s
        .lines()
        .filter(|l| !(l.contains("ab") || l.contains("cd") || l.contains("pq") || l.contains("xy")))
        .filter(|l| {
            l.trim()
                .chars()
                .fold((false, '0'), |state, w| match state {
                    (true, _) => state,
                    (false, prev) => {
                        if w == prev {
                            (true, w)
                        } else {
                            (false, w)
                        }
                    }
                })
                .0
        })
        .filter(|l| {
            let vowels = l.chars().fold(0, |acc, c| match c {
                'a' | 'e' | 'i' | 'o' | 'u' => acc + 1,
                _ => acc,
            });
            vowels > 2
        })
        .collect::<Vec<&str>>();
    s.chars().fold(0, |acc, c| match c {
        'a' | 'e' | 'i' | 'o' | 'u' => acc + 1,
        _ => acc,
    });
    lines.len()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    const TESTINPUT: &str = "ugknbfddgicrmopn
aaa
jchzalrnumimnmhp
haegwjzuvuyypxyu
dvszwmarrgswjxmb";

    #[test]
    fn test_part1() {
        let expected: usize = 2;
        let actual: usize = part1(TESTINPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_with_input() {
        let expected: usize = 236;
        let actual: usize = part1(include_str!["input.txt"]);
        assert_eq!(actual, expected);
    }
}
