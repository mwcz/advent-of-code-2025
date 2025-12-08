//! A solution to day 2 year 2025.
//! https://adventofcode.com/2025/day/2

use crate::digits::count::digit_count;
use std::ops::RangeInclusive;

type Model = Vec<(i64, i64)>;
type Answer = i64;

pub fn parse(input: String) -> Model {
    input
        .split(',')
        .map(|r| {
            let (a, b) = r.split_once('-').unwrap();
            (a.parse().unwrap(), b.trim_end().parse().unwrap())
        })
        .collect::<Vec<_>>()
}

fn is_invalid_p1(n: i64) -> bool {
    let dc = digit_count(n);
    if dc.is_multiple_of(2) {
        let upper = n / 10_i64.pow(dc / 2);
        let lower = n - upper * 10_i64.pow(dc / 2);

        if upper == lower {
            return true;
        }
    }
    false
}

pub fn part1(model: Model) -> Answer {
    let mut sum = 0;
    for (id1, id2) in model {
        for id in id1..=id2 {
            if is_invalid_p1(id) {
                sum += id;
            }
        }
    }

    sum
}

fn is_invalid_p2(n: i64) -> bool {
    let dc = digit_count(n);
    for repeats in 2..=dc {
        if dc.is_multiple_of(repeats) {
            let patlen = dc / repeats;
            let pat = n / 10_i64.pow(patlen * (repeats - 1));

            let mut val = pat;

            for _ in 1..repeats {
                if let Some(tens) = 10_i64.checked_pow(patlen) {
                    if let Some(new_val) = val.checked_mul(tens) {
                        val = new_val;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
                val += pat;
            }
            // for _ in 0..repeats {
            //     val -= pat;
            //     val /= 10_i64.pow(repeats);
            // }

            if val == n {
                return true;
            }
        }
    }
    false
}

pub fn part2(model: Model) -> Answer {
    let mut sum = 0;
    for (id1, id2) in model {
        for id in id1..=id2 {
            if is_invalid_p2(id) {
                sum += id;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d2");
    const EXAMPLE: &str = include_str!("../examples/d2");

    #[test]
    fn d2p1_example_test() {
        assert_eq!(part1(parse(EXAMPLE.to_string())), 1227775554);
    }

    #[test]
    fn d2p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 28846518423);
    }

    #[test]
    fn d2p2_example_test() {
        assert_eq!(part2(parse(EXAMPLE.to_string())), 4174379265);
    }

    #[test]
    fn d2p2_input_test() {
        assert_eq!(part2(parse(INPUT.to_string())), 31578210022);
    }
}
