use lazy_static::lazy_static;
use regex::Regex;

static INPUT: &str = include_str!("input.txt");
lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
}

fn main() {
    let input = advent::get_input!();
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let mut output: usize = 0;
    for l in input.lines() {
        let (min, max, letter, pass) = parse(l);
        let count = pass.chars().filter(|c| *c == letter).count();
        if count >= min && count <= max {
            output += 1;
        }
    }
    output
}

fn part_2(input: &str) -> usize {
    let mut output: usize = 0;
    for l in input.lines() {
        let (first, second, letter, pass) = parse(l);
        let first = letter == pass.chars().nth(first-1).unwrap();
        let second = letter == pass.chars().nth(second-1).unwrap();
        if first != second {
            output += 1;
        }
    }
    output
}

fn parse(s: &str) -> (usize, usize, char, &str) {
    let caps = RE.captures(s).unwrap();
    let pass = &caps.get(4).unwrap();
    (
        caps[1].parse().unwrap(),
        caps[2].parse().unwrap(),
        caps[3].parse().unwrap(),
        &s[pass.start()..pass.end()]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(655, part_1(INPUT));
    }
    
    #[test]
    fn test_part_2() {
        assert_eq!(673, part_2(INPUT));
    }
}
