use aoc_rust_common::Solution;
use std::fmt::Display;

pub struct Day07;

fn get_sum_of_until(until: i64) -> i64 {
    until * (until + 1) / 2
}

impl Solution for Day07 {
    fn year(&self) -> u32 {
        2021
    }
    fn day(&self) -> u32 {
        7
    }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let numbers: Vec<i64> = input
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let min = *numbers.iter().min().unwrap();
        let max = *numbers.iter().max().unwrap();

        let min_fuel = (min..=max)
            .map(|i| numbers.iter().map(|&n| (n - i).abs()).sum::<i64>())
            .min()
            .unwrap();

        Box::new(min_fuel)
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let numbers: Vec<i64> = input
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let min = *numbers.iter().min().unwrap();
        let max = *numbers.iter().max().unwrap();

        let min_fuel = (min..=max)
            .map(|i| {
                numbers
                    .iter()
                    .map(|&n| get_sum_of_until((n - i).abs()))
                    .sum::<i64>()
            })
            .min()
            .unwrap();

        Box::new(min_fuel)
    }
}
