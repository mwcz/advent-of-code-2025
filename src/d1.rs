//! A solution to day 1 year 2025.
//! https://adventofcode.com/2025/day/1

type Model = Vec<Turn>;
type Answer = i64;

#[derive(Debug)]
pub enum Turn {
    Left(i64),
    Right(i64),
}

pub fn parse(input: String) -> Model {
    input
        .lines()
        .map(|line| match line.split_at(1) {
            ("L", amt) => Turn::Left(amt.parse().unwrap()),
            ("R", amt) => Turn::Right(amt.parse().unwrap()),
            _ => panic!("bad input"),
        })
        .collect::<Vec<_>>()
}

pub fn part1(model: Model) -> Answer {
    let mut dial = 50;
    let mut zc = 0;

    for turn in &model {
        dial += match turn {
            Turn::Left(amt) => -*amt,
            Turn::Right(amt) => *amt,
        };
        dial %= 100;
        if dial == 0 {
            zc += 1;
        }
    }

    zc
}

pub fn part2(model: Model) -> Answer {
    let mut dial = 50;
    let mut zc = 0;

    for turn in &model {
        let amt = match turn {
            Turn::Left(amt) => -*amt,
            Turn::Right(amt) => *amt,
        };
        for _ in 0..amt.abs() {
            dial += amt.signum();
            dial %= 100;
            if dial == 0 {
                zc += 1;
            }
        }
    }

    zc
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d1");
    const EXAMPLE: &str = include_str!("../examples/d1");

    #[test]
    fn d1p1_example_test() {
        assert_eq!(part1(parse(EXAMPLE.to_string())), 3);
    }

    #[test]
    fn d1p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 1191);
    }

    #[test]
    fn d1p2_example_test() {
        assert_eq!(part2(parse(EXAMPLE.to_string())), 6);
    }

    #[test]
    fn d1p2_input_test() {
        assert_eq!(part2(parse(INPUT.to_string())), 6858);
    }
}
