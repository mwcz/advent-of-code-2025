//! A solution to day 3 year 2025.
//! https://adventofcode.com/2025/day/3

use std::fmt::Debug;

use itertools::Itertools as _;
use num_bigint::BigInt;

use crate::digits::crud::{get_digit, set_digit};

type Model = Vec<Vec<u8>>;
type Answer = u64;

pub fn parse(input: String) -> Model {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

pub fn part1(model: Model) -> Answer {
    let mut sum = 0;
    for bank in &model {
        let max_pos = bank.len() - bank.iter().rev().position_max().unwrap() - 1;
        let max = bank[max_pos];
        if max_pos < bank.len() - 1 {
            let max_after = bank[max_pos + 1..].iter().max().unwrap();
            sum += max * 10 + max_after;
        } else {
            let max_before = bank[0..max_pos].iter().max().unwrap();
            sum += max_before * 10 + max;
        }
    }

    sum.into()
}

// fn rmax(bank: &[u64], rem: u8, sum: u64) -> u64 {
//     if rem == 0 || bank.is_empty() {
//         sum
//     } else if bank.len() == 1 {
//         sum + bank[0]
//     } else {
//         let max_pos = bank.len() - bank.iter().rev().position_max().unwrap() - 1;
//         let max = bank[max_pos];
//         if max_pos < bank.len() - 1 {
//             // let max_after = bank[max_pos + 1..].iter().max().unwrap();
//             // sum += max * 10 + max_after;
//             let new_sum = sum * 100 + max * 10 + max_after;
//             rmax(&bank[max_pos + 1..], rem - 1, new_sum)
//         } else {
//             // let max_before = bank[0..max_pos].iter().max().unwrap();
//             let dc = digit_count_u64(sum);
//             let new_sum = max_before * 10_u64.pow(dc + 2) + sum;
//             rmax(&bank[0..max_pos], rem - 1, new_sum)
//         }
//     }
// }

enum Candidate<'a> {
    Digit(u8),
    Slice(&'a [u8]),
}

impl<'a> Candidate<'a> {
    /// Returns `true` if the candidate is [`Slice`].
    ///
    /// [`Slice`]: Candidate::Slice
    #[must_use]
    fn is_slice(&self) -> bool {
        matches!(self, Self::Slice(..))
    }

    /// Returns `true` if the candidate is [`Digit`].
    ///
    /// [`Digit`]: Candidate::Digit
    #[must_use]
    fn is_digit(&self) -> bool {
        matches!(self, Self::Digit(..))
    }

    fn as_digit(&self) -> Option<u8> {
        if let Self::Digit(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    fn as_slice(&self) -> Option<&'a [u8]> {
        if let Self::Slice(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

/// Find the maximum joltage available from this bank of batteries.
fn max_jolt(bank: Vec<u8>, bat_count: u32) -> u64 {
    let mut c = vec![Candidate::Slice(&bank)];
    let mut d = 12;

    while d > 0 {
        if let Some((i, s)) = c.iter().enumerate().rfind(|(i, can)| can.is_slice()) {
            let s = s.as_slice().unwrap();

            // handle "this slice finishes the answer" caase
            if s.len() == d {
                c.remove(i);
                for num in s.iter().rev() {
                    c.insert(i, Candidate::Digit(*num));
                }
                d = 0;
                break;
            }

            let (m_i, m) = s
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.cmp(b))
                .unwrap();
            // keep following the algo
        }
    }

    let digits = c.into_iter().filter_map(|can| can.as_digit());

    let mut sum = 0;

    for (i, digit) in digits.enumerate() {
        let mag = 12 - i;
        sum += 10u64.pow(mag as u32) * digit as u64;
    }

    sum
}

/*

Algorithm ideas


818181911112111

Let C be the candidate digits and ranges
Let D be the count of digits we need to complete

For the example
    C = [[818181911112111]]
    D = 12

1. If D == 0, take the non-range digits from C to form the answer
2. If D > 0 then
3. Take the rightmost range R within C with length > 1               (ex: [818181911112111])
4. If R is the rightmost range and the length of R == D, then flatten the digits of R directly into C which is now the answer
5. Find the maximum digit m within R (the rightmost max if tied)    (ex: 9)
6. Split R on m                                                     (ex: R = [[818181], 9, [11112111]])
7. Replace the R in C with the split R                              (ex: C = [[818181], 9, [11112111]] (flatten it))
8. Decrement D by 1                                                 (ex: D = 11)

Possible alterations:
 - Step 3: take the leftmost range
 - Step 3: remove the length > 1 constraint

Testing it 818181911112111

    C = [[818181911112111]]
    R = [818181911112111]
    D = 12

    C = [[818181], 9, [11112111]]
    R = [818181]
    D = 11

    C = [[8181], 8, [1], 9, [11112111]]
    R = [8181]
    D = 10

    C = [[81], 8, [1], 8, [1], 9, [11112111]]
    R = [81]
    D = 9

    C = [8, [1], 8, [1], 8, [1], 9, [11112111]]
    R = [11112111]
    D = 8

    len(R) == D, so flatten R into C

    C = [8, [1], 8, [1], 8, [1], 9, 1, 1, 1, 1, 2, 1, 1, 1]
    D = 0

    A = 888911112111

Testing it with 811111111111119

    C = [[811111111111119]]
    D = 12

    C = [[81111111111111], 9]
    D = 11

    C = [8, [1111111111111], 9]
    D = 10

    C = [8, [111111111111], 1, 9]
    D = 9

    C = [8, [11111111111], 1, 1, 9]
    D = 8

    C = [8, [1111111111], 1, 1, 1, 9]
    D = 7

    C = [8, [111111111], 1, 1, 1, 1, 9]
    D = 6

    C = [8, [11111111], 1, 1, 1, 1, 1, 9]
    D = 5

    C = [8, [1111111], 1, 1, 1, 1, 1, 1, 9]
    D = 4

    C = [8, [111111], 1, 1, 1, 1, 1, 1, 1, 9]
    D = 3

    C = [8, [11111], 1, 1, 1, 1, 1, 1, 1, 1, 9]
    D = 2

    C = [8, [1111], 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]
    D = 1

    C = [8, [111], 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]
    D = 0

    C = [8, [111], 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]
    A = 811111111119

Testing with 987654321111111

    C = [[987654321111111]]
    D = 12

    C = [9, [87654321111111]]
    D = 11

    C = [9, 8, [7654321111111]]
    D = 10

    C = [9, 8, 7, [654321111111]]
    D = 9

    C = [9, 8, 7, 6, [54321111111]]
    D = 8

    C = [9, 8, 7, 6, 5, [4321111111]]
    D = 7

    C = [9, 8, 7, 6, 5, 4, [321111111]]
    D = 6

    C = [9, 8, 7, 6, 5, 4, 3, [21111111]]
    D = 5

    C = [9, 8, 7, 6, 5, 4, 3, 2, [1111111]]
    D = 4

    C = [9, 8, 7, 6, 5, 4, 3, 2, [111111], 1]
    D = 3

    C = [9, 8, 7, 6, 5, 4, 3, 2, [11111], 1, 1]
    D = 2

    C = [9, 8, 7, 6, 5, 4, 3, 2, [1111], 1, 1, 1]
    D = 1

    C = [9, 8, 7, 6, 5, 4, 3, 2, [111], 1, 1, 1, 1]
    D = 0

    A = 987654321111

Testing with 234234234234278

    C = [[234234234234278]]
    D = 12

    C = [[23423423423427], 8]
    D = 11

    C = [[2342342342342], 7, 8]
    D = 10

    C = [[23423423423], 4, [2], 7, 8]
    D = 9

    C = [[23423423], 4, [23], 4, [2], 7, 8]
    D = 8

    C = [[23423], 4, [23], 4, [23], 4, [2], 7, 8]
    D = 7

    C = [[23], 4, [23], 4, [23], 4, [23], 4, [2], 7, 8]
    D = 6

    C = [[23], 4, [23], 4, [23], 4, [23], 4, [2], 7, 8]
    D = 6

*/

pub fn part2(model: Model) -> Answer {
    let mut sum = 0;

    for bank in model {
        sum += max_jolt(bank, 12);
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
    //         part2(INPUT.to_string()),
    //         "put part 2 final answer here"
    //     );
    // }
}
