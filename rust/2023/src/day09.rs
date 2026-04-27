use anyhow::Result;
use aoc_rust_common::Solution;

pub struct Day09;

fn extrapolate(xs: &[i64]) -> i64 {
    if xs.is_empty() {
        return 0;
    }
    if xs.iter().all(|&x| x == 0) {
        return 0;
    }
    let next_xs: Vec<i64> = xs.windows(2).map(|w| w[1] - w[0]).collect();
    xs.last().unwrap() + extrapolate(&next_xs)
}

impl Solution for Day09 {
    fn year(&self) -> u32 {
        2023
    }
    fn day(&self) -> u32 {
        9
    }

    fn part1(&self, input: &str) -> Result<String> {
        let result: i64 = input
            .lines()
            .map(|line| {
                let xs: Vec<i64> = line
                    .split_whitespace()
                    .map(|s| s.parse().expect("failed to parse number"))
                    .collect();
                extrapolate(&xs)
            })
            .sum();
        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let result: i64 = input
            .lines()
            .map(|line| {
                let mut xs: Vec<i64> = line
                    .split_whitespace()
                    .map(|s| s.parse().expect("failed to parse number"))
                    .collect();
                xs.reverse();
                extrapolate(&xs)
            })
            .sum();
        Ok(result.to_string())
    }
}

aoc_rust_common::aoc_test!(Day09, "2101499000", "1089");
