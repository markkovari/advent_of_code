use crate::utils::intcode::{growing_memory, VM};
use anyhow::Result;
use aoc_rust_common::Solution;

pub struct Day05;

impl Solution for Day05 {
    fn year(&self) -> u32 {
        2019
    }
    fn day(&self) -> u32 {
        5
    }

    fn part1(&self, input: &str) -> Result<String> {
        let memory: Vec<i64> = input
            .trim()
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect();
        let mut vm = VM::new(growing_memory(memory));
        let mut outputs = Vec::new();
        vm.run_all(|| Ok(1), |v| {
            outputs.push(v);
            Ok(())
        })?;
        Ok(outputs.last().unwrap().to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let memory: Vec<i64> = input
            .trim()
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect();
        let mut vm = VM::new(growing_memory(memory));
        let mut outputs = Vec::new();
        vm.run_all(|| Ok(5), |v| {
            outputs.push(v);
            Ok(())
        })?;
        Ok(outputs.last().unwrap().to_string())
    }
}

aoc_rust_common::aoc_test!(Day05);
