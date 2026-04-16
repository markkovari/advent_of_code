use aoc_rust_common::Solution;
use std::fmt::Display;

pub struct Day25;

fn snafu_to_decimal(snafu: &str) -> i64 {
    snafu.chars().rev().enumerate().fold(0, |acc, (i, c)| {
        let val = match c {
            '0' => 0, '1' => 1, '2' => 2, '-' => -1, '=' => -2,
            _ => 0,
        };
        acc + val * 5_i64.pow(i as u32)
    })
}

fn decimal_to_snafu(mut number: i64) -> String {
    let mut digits = Vec::new();
    while number != 0 {
        let mut rem = ((number + 2) % 5) - 2;
        number = (number - rem) / 5;
        digits.push(match rem {
            0 => '0', 1 => '1', 2 => '2', -1 => '-', -2 => '=',
            _ => unreachable!(),
        });
    }
    digits.iter().rev().collect()
}

impl Solution for Day25 {
    fn year(&self) -> u32 { 2022 }
    fn day(&self) -> u32 { 25 }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let total = input.lines().map(snafu_to_decimal).sum();
        Box::new(decimal_to_snafu(total))
    }

    fn part2(&self, _input: &str) -> Box<dyn Display> {
        Box::new("There is no part 2 for Day 25!")
    }
}
