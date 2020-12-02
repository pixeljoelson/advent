use itertools::Itertools;
use timed::timed;

fn main() {
    let input: Vec<u32> = include_str!("input.txt")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();
    solve_manual(&input);
    solve_auto(&input);
}

#[timed]
fn solve_manual(input: &[u32]) {
    println!("{:?}", solve(&input));
    println!("{:?}", solve2(&input));
}
#[timed]
fn solve_auto(input: &[u32]) {
    println!("{:?}", solve_generic(&input, 2));
    println!("{:?}", solve_generic(&input, 3));
}

fn solve(input: &[u32]) -> Option<u32> {
    for x in input.iter() {
        for y in input.iter() {
            if x + y == 2020 {
                return Some(x * y);
            }
        }
    }
    None
}

fn solve2(input: &[u32]) -> Option<u32> {
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

fn solve_generic(input: &[u32], n: usize) -> Option<u32> {
    for v in input.iter().combinations(n) {
        if v.iter().fold(0, |acc, x| acc + *x) == 2020 {
            return Some(v.iter().fold(1, |acc, x| acc * *x));
        }
    }
    None
}
