use anyhow::Result;
use aoc_rust_common::Solution;
use std::fmt::Display;
use std::collections::HashSet;

pub struct Day03;

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => 0,
    }
}

impl Solution for Day03 {
    fn year(&self) -> u32 { 2022 }
    fn day(&self) -> u32 { 3 }

    fn part1(&self, input: &str) -> Result<String> {
        let sum: u32 = input.lines().map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            let set1: HashSet<char> = first.chars().collect();
            let set2: HashSet<char> = second.chars().collect();
            let common = set1.intersection(&set2).next().unwrap();
            priority(*common)
        }).sum();
        Ok((sum).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let lines: Vec<&str> = input.lines().collect();
        let sum: u32 = lines.chunks(3).map(|chunk| {
            let set1: HashSet<char> = chunk[0].chars().collect();
            let set2: HashSet<char> = chunk[1].chars().collect();
            let set3: HashSet<char> = chunk[2].chars().collect();
            
            let common: HashSet<char> = set1.intersection(&set2).cloned().collect();
            let badge = *common.intersection(&set3).next().unwrap();
            priority(badge)
        }).sum();
        Ok((sum).to_string())
    }
}
