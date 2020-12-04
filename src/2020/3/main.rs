static INPUT: &str = include_str!("input.txt");

fn main() {
    let input = advent::get_input!();

    let (input, width) = parse(&input);
    println!("{}", part_1(&input, width, (3, 1)));
    println!(
        "{}",
        part_2(&input, width, &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)])
    );
}

fn parse(s: &str) -> (Vec<u8>, usize) {
    (
        s.bytes().filter(|b| b".#".contains(b)).collect(),
        s.lines().nth(1).unwrap().bytes().count(),
    )
}

fn part_1(input: &[u8], width: usize, slope: (usize, usize)) -> usize {
    let (slope_x, slope_y) = slope;
    let (mut x, mut y) = slope;
    let mut trees = 0;
    while let Some(b) = input.get(y * width + x) {
        if *b == b'#' {
            trees += 1;
        }
        y += slope_y;
        x = (x + slope_x) % width;
    }
    trees
}

fn part_2(input: &[u8], width: usize, slopes: &[(usize, usize)]) -> usize {
    let mut results: Vec<usize> = vec![];
    for s in slopes.iter() {
        results.push(part_1(input, width, *s));
    }
    results.iter().fold(1usize, |acc, n| acc * *n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let (input, width) = parse(INPUT);
        assert_eq!(220, part_1(&input, width, (3, 1)));
    }

    #[test]
    fn test_part_2() {
        let (input, width) = parse(INPUT);
        assert_eq!(
            2138320800,
            part_2(&input, width, &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)])
        );
    }
}
