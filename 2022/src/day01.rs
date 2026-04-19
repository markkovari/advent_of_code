use anyhow::Result;
use aoc_rust_common::Solution;

pub struct Day01;

impl Solution for Day01 {
    fn year(&self) -> u32 {
        2022
    }
    fn day(&self) -> u32 {
        1
    }

    fn part1(&self, input: &str) -> Result<String> {
        let max_sum = input
            .split("\n\n")
            .map(|block| {
                block
                    .lines()
                    .filter_map(|line| line.parse::<i32>().ok())
                    .sum::<i32>()
            })
            .max()
            .unwrap_or(0);
        Ok((max_sum).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut sums: Vec<i32> = input
            .split("\n\n")
            .map(|block| {
                block
                    .lines()
                    .filter_map(|line| line.parse::<i32>().ok())
                    .sum::<i32>()
            })
            .collect();
        sums.sort_by(|a, b| b.cmp(a));
        Ok((sums.iter().take(3).sum::<i32>()).to_string())
    }
}
