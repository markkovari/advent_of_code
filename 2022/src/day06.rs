use anyhow::Result;
use aoc_rust_common::Solution;
use std::collections::HashSet;

pub struct Day06;

fn are_unique(elements: &[char]) -> bool {
    let mut unique_elements: HashSet<char> = HashSet::new();
    for &element in elements {
        if !unique_elements.insert(element) {
            return false;
        }
    }
    true
}

fn solve(input: &str, window_size: usize) -> usize {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(window_size)
        .position(|w| are_unique(w))
        .unwrap()
        + window_size
}

impl Solution for Day06 {
    fn year(&self) -> u32 {
        2022
    }
    fn day(&self) -> u32 {
        6
    }

    fn part1(&self, input: &str) -> Result<String> {
        Ok((solve(input, 4)).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        Ok((solve(input, 14)).to_string())
    }
}

aoc_rust_common::aoc_test!(Day06);
