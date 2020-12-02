use itertools::Itertools;
use timed::timed;

static INPUT: &str = include_str!("input.txt");

fn main() {
    let input: Vec<u32> = advent::get_input!()
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();
    solve_manual(&input);
    solve_auto(&input);
}

#[timed]
fn solve_manual(input: &[u32]) {
    println!("{:?}", part_1(&input));
    println!("{:?}", part_2(&input));
}
#[timed]
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
