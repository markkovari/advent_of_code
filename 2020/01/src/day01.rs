use aoc_rust_common::Solution;
use std::fmt::Display;

pub struct Day01;

impl Solution for Day01 {
    fn year(&self) -> u32 { 2020 }
    fn day(&self) -> u32 { 1 }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let numbers: Vec<u32> = input.lines().filter_map(|s| s.parse().ok()).collect();
        for i in 0..numbers.len() {
            for j in i + 1..numbers.len() {
                if numbers[i] + numbers[j] == 2020 {
                    return Box::new(numbers[i] * numbers[j]);
                }
            }
        }
        Box::new(0)
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let numbers: Vec<u32> = input.lines().filter_map(|s| s.parse().ok()).collect();
        for i in 0..numbers.len() {
            for j in i + 1..numbers.len() {
                for k in j + 1..numbers.len() {
                    if numbers[i] + numbers[j] + numbers[k] == 2020 {
                        return Box::new(numbers[i] * numbers[j] * numbers[k]);
                    }
                }
            }
        }
        Box::new(0)
    }
}
