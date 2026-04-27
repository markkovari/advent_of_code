use anyhow::Result;
use aoc_rust_common::Solution;
use std::collections::HashSet;

pub struct Day04;

fn count_matches(line: &str) -> usize {
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() < 2 {
        return 0;
    }
    let numbers_part = parts[1];
    let number_sets: Vec<&str> = numbers_part.split('|').collect();
    if number_sets.len() < 2 {
        return 0;
    }

    let winning_numbers: HashSet<i32> = number_sets[0]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    let given_numbers: HashSet<i32> = number_sets[1]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    winning_numbers.intersection(&given_numbers).count()
}

impl Solution for Day04 {
    fn year(&self) -> u32 {
        2023
    }
    fn day(&self) -> u32 {
        4
    }

    fn part1(&self, input: &str) -> Result<String> {
        let result: u32 = input
            .lines()
            .map(|line| {
                let matches = count_matches(line);
                if matches > 0 {
                    2u32.pow((matches - 1) as u32)
                } else {
                    0
                }
            })
            .sum();
        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let lines: Vec<&str> = input.lines().collect();
        let n = lines.len();
        let mut counts = vec![1u32; n];

        for (i, line) in lines.iter().enumerate() {
            let matches = count_matches(line);
            let current_count = counts[i];
            for j in 1..=matches {
                if i + j < n {
                    counts[i + j] += current_count;
                }
            }
        }

        let total: u32 = counts.iter().sum();
        Ok(total.to_string())
    }
}

aoc_rust_common::aoc_test!(Day04, "22193", "5625994");
