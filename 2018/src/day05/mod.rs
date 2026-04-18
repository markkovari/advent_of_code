use aoc_rust_common::Solution;
use std::fmt::Display;

pub struct Day05;

fn react(polymer: &mut Vec<char>) {
    let mut i = 0;
    while i < polymer.len().saturating_sub(1) {
        if polymer[i].eq_ignore_ascii_case(&polymer[i + 1]) && polymer[i] != polymer[i + 1] {
            polymer.remove(i);
            polymer.remove(i);
            i = i.saturating_sub(1);
        } else {
            i += 1;
        }
    }
}

impl Solution for Day05 {
    fn year(&self) -> u32 {
        2018
    }
    fn day(&self) -> u32 {
        5
    }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let mut polymer: Vec<char> = input.trim().chars().collect();
        react(&mut polymer);
        Box::new(polymer.len())
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let original_polymer: Vec<char> = input.trim().chars().collect();
        let min_len = ('a'..='z')
            .map(|unit_to_remove| {
                let mut polymer = original_polymer.clone();
                polymer.retain(|&c| !c.eq_ignore_ascii_case(&unit_to_remove));
                react(&mut polymer);
                polymer.len()
            })
            .min()
            .unwrap_or(0);
        Box::new(min_len)
    }
}
