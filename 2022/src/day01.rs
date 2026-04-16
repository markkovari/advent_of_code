use aoc_rust_common::Solution;
use std::fmt::Display;

pub struct Day01;

impl Solution for Day01 {
    fn year(&self) -> u32 { 2022 }
    fn day(&self) -> u32 { 1 }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let max_sum = input.split("\n\n")
            .map(|block| block.lines().filter_map(|line| line.parse::<i32>().ok()).sum::<i32>())
            .max().unwrap_or(0);
        Box::new(max_sum)
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let mut sums: Vec<i32> = input.split("\n\n")
            .map(|block| block.lines().filter_map(|line| line.parse::<i32>().ok()).sum::<i32>())
            .collect();
        sums.sort_by(|a, b| b.cmp(a));
        Box::new(sums.iter().take(3).sum::<i32>())
    }
}
