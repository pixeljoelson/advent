use sugars::hset;

static INPUT: &str = include_str!("input.txt");

fn main() {
    let input = advent::get_input!();

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn parse(input: &str) -> String {
    input.chars().filter(|&c| c != '\r').collect::<String>()
}

fn part_1(input: &str) -> usize {
    let mut valid: usize = 0;
    'outer: for s in parse(input).split("\n\n") {
        for f in &[
            "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", // "cid",
        ] {
            if !s.contains(f) {
                continue 'outer;
            }
        }
        valid += 1;
    }
    valid
}

fn part_2(input: &str) -> usize {
    let mut valid: usize = 0;
    for s in parse(input).split("\n\n") {
        let required_fields = hset! {
            "byr", "iyr", "eyr",
            "hgt", "hcl", "ecl",
            "pid", // "cid",
        };
        let mut valid_fields = hset! {};
        for entry in s.split_whitespace() {
            let field = &entry[..3];
            let val = &entry[4..];
            if match field {
                "byr" => val.parse::<u32>().map_or(false, |y| y >= 1920 && y <= 2002),
                "iyr" => val.parse::<u32>().map_or(false, |y| y >= 2010 && y <= 2020),
                "eyr" => val.parse::<u32>().map_or(false, |y| y >= 2020 && y <= 2030),
                "hgt" => {
                    val.len() >= 3 && {
                        if let Ok(n) = val
                            .trim_end_matches("cm")
                            .trim_end_matches("in")
                            .parse::<u32>()
                        {
                            if val.ends_with("cm") {
                                n >= 150 && n <= 193
                            } else {
                                n >= 59 && n <= 76
                            }
                        } else {
                            false
                        }
                    }
                }
                "hcl" => {
                    let mut chars = val.chars();
                    val.len() == 7
                        && chars.next().map_or(false, |c| c == '#')
                        && chars.all(|c| (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f'))
                }
                "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&val),
                "pid" => val.len() == 9 && val.parse::<u32>().is_ok(),
                _ => true,
            } {
                valid_fields.insert(field);
            }
        }
        if valid_fields.is_superset(&required_fields) {
            valid += 1;
        }
    }
    valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(202, part_1(INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(137, part_2(INPUT));
    }
}
