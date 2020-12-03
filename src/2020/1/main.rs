use itertools::Itertools;

static INPUT: &str = include_str!("input.txt");

fn main() {
    let input: Vec<u32> = parse(&advent::get_input!());
        
    solve_manual(&input);
    solve_auto(&input);
}

fn parse(s: &str) -> Vec<u32> {
    s.lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn solve_manual(input: &[u32]) {
    println!("{:?}", part_1(&input));
    println!("{:?}", part_2(&input));
}

fn solve_auto(input: &[u32]) {
    println!("{:?}", generic(&input, 2));
    println!("{:?}", generic(&input, 3));
}

fn part_1(input: &[u32]) -> Option<u32> {
    for x in input.iter() {
        for y in input.iter() {
            if x + y == 2020 {
                return Some(x * y);
            }
        }
    }
    None
}

fn part_2(input: &[u32]) -> Option<u32> {
    for x in input.iter() {
        for y in input.iter() {
            for z in input.iter() {
                if x + y + z == 2020 {
                    return Some(x * y * z);
                }
            }
        }
    }
    None
}

fn generic(input: &[u32], n: usize) -> Option<u32> {
    for v in input.iter().combinations(n) {
        if v.iter().fold(0, |acc, x| acc + *x) == 2020 {
            return Some(v.iter().fold(1, |acc, x| acc * *x));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(1019571, part_1(&parse(INPUT)).unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(100655544, part_2(&parse(INPUT)).unwrap());
    }
}