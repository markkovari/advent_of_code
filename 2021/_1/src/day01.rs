use anyhow::Result;
use aoc_rust_common::Solution;

pub struct Day01;

impl Solution for Day01 {
    fn year(&self) -> u32 {
        2021
    }
    fn day(&self) -> u32 {
        1
    }

    fn part1(&self, input: &str) -> Result<String> {
        let numbers: Vec<i32> = input.lines().filter_map(|l| l.parse().ok()).collect();
        let increments = numbers.windows(2).filter(|w| w[1] > w[0]).count();
        Ok((increments).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let numbers: Vec<i32> = input.lines().filter_map(|l| l.parse().ok()).collect();
        let sums: Vec<i32> = numbers.windows(3).map(|w| w.iter().sum()).collect();
        let increments = sums.windows(2).filter(|w| w[1] > w[0]).count();
        Ok((increments).to_string())
    }
}
