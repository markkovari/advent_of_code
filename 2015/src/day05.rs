use anyhow::Result;
use aoc_rust_common::Solution;
use rayon::prelude::*;

pub struct Day05;

impl Solution for Day05 {
    fn year(&self) -> u32 {
        2015
    }
    fn day(&self) -> u32 {
        5
    }

    fn part1(&self, input: &str) -> Result<String> {
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        let filters = ["ab", "cd", "pq", "xy"];

        Ok((input
            .par_lines()
            .filter(|line| line.chars().filter(|c| vowels.contains(c)).count() >= 3)
            .filter(|line| {
                let chars: Vec<char> = line.chars().collect();
                chars.windows(2).any(|w| w[0] == w[1])
            })
            .filter(|line| !filters.iter().any(|&f| line.contains(f)))
            .count() as i64)
            .to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        Ok((input
            .par_lines()
            .filter(|line| {
                let chars: Vec<char> = line.chars().collect();
                chars.windows(3).any(|w| w[0] == w[2])
            })
            .filter(|line| {
                let chars: Vec<char> = line.chars().collect();
                (0..chars.len() - 1).any(|i| {
                    let pair = &line[i..i + 2];
                    line[i + 2..].contains(pair)
                })
            })
            .count() as i64)
            .to_string())
    }
}

aoc_rust_common::aoc_test!(Day05, "1", "1");
