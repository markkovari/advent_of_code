use anyhow::Result;
use aoc_rust_common::Solution;

pub struct Day01;

impl Solution for Day01 {
    fn year(&self) -> u32 {
        2020
    }
    fn day(&self) -> u32 {
        1
    }

    fn part1(&self, input: &str) -> Result<String> {
        let numbers: Vec<u32> = input.lines().filter_map(|s| s.parse().ok()).collect();
        for i in 0..numbers.len() {
            for j in i + 1..numbers.len() {
                if numbers[i] + numbers[j] == 2020 {
                    return Ok((numbers[i] * numbers[j]).to_string());
                }
            }
        }
        Ok((0).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let numbers: Vec<u32> = input.lines().filter_map(|s| s.parse().ok()).collect();
        for i in 0..numbers.len() {
            for j in i + 1..numbers.len() {
                for k in j + 1..numbers.len() {
                    if numbers[i] + numbers[j] + numbers[k] == 2020 {
                        return Ok((numbers[i] * numbers[j] * numbers[k]).to_string());
                    }
                }
            }
        }
        Ok((0).to_string())
    }
}

aoc_rust_common::aoc_test!(Day01);
