//! A solution to day 3 year 2025.
//! https://adventofcode.com/2025/day/3

use std::fmt::Debug;

use itertools::Itertools as _;

use crate::digits::digit_count_u64;

type Model = Vec<Vec<u64>>;
type Answer = u64;

pub fn parse(input: String) -> Model {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap().into())
                .collect()
        })
        .collect()
}

pub fn part1(model: Model) -> Answer {
    let mut sum = 0;
    for bank in &model {
        println!("================");
        println!("{bank:?}");
        let max_pos = bank.len() - bank.iter().rev().position_max().unwrap() - 1;
        dbg!(max_pos);
        let max = bank[max_pos];
        println!("max: {max} @ {max_pos}");
        if max_pos < bank.len() - 1 {
            let max_after = bank[max_pos + 1..].iter().max().unwrap();
            sum += max * 10 + max_after;
            println!("{:?} {}", &bank, max * 10 + max_after);
        } else {
            let max_before = bank[0..max_pos].iter().max().unwrap();
            sum += max_before * 10 + max;
            println!("{:?} {}", &bank, max_before * 10 + max);
        }
    }

    sum
}

fn rmax(bank: &[u64], rem: u8, sum: u64) -> u64 {
    if rem == 0 || bank.is_empty() {
        sum
    } else if bank.len() == 1 {
        sum + bank[0]
    } else {
        let max_pos = bank.len() - bank.iter().rev().position_max().unwrap() - 1;
        let max = bank[max_pos];
        if max_pos < bank.len() - 1 {
            // let max_after = bank[max_pos + 1..].iter().max().unwrap();
            // sum += max * 10 + max_after;
            let new_sum = sum * 100 + max * 10 + max_after;
            rmax(&bank[max_pos + 1..], rem - 1, new_sum)
        } else {
            // let max_before = bank[0..max_pos].iter().max().unwrap();
            let dc = digit_count_u64(sum);
            let new_sum = max_before * 10_u64.pow(dc + 2) + sum;
            rmax(&bank[0..max_pos], rem - 1, new_sum)
        }
    }
}

pub fn part2(model: Model) -> Answer {
    let mut sum = 0;

    for bank in &model {
        sum += rmax(bank, 12, 0);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d3");
    const EXAMPLE: &str = include_str!("../examples/d3");

    #[test]
    fn d3p1_example_test() {
        assert_eq!(part1(parse(EXAMPLE.to_string())), 357);
    }

    #[test]
    fn d3p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 16993);
    }

    #[test]
    fn d3p2_example_test() {
        assert_eq!(part2(parse(EXAMPLE.to_string())), 3121910778619);
    }

    // #[test]
    // fn d3p2_input_test() {
    //     assert_eq!(
    //         part2(parse(INPUT.to_string())),
    //         "put part 2 final answer here"
    //     );
    // }
}
