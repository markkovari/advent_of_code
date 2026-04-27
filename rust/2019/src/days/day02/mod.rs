use crate::utils::intcode::{growing_memory, VM};
use anyhow::Result;
use aoc_rust_common::Solution;

pub struct Day02;

fn run_with_params(memory: &[i64], noun: i64, verb: i64) -> i64 {
    let mut mem = memory.to_vec();
    mem[1] = noun;
    mem[2] = verb;
    let mut vm = VM::new(growing_memory(mem));
    vm.run_all_no_io().unwrap();
    vm.memory.0[0]
}

impl Solution for Day02 {
    fn year(&self) -> u32 {
        2019
    }
    fn day(&self) -> u32 {
        2
    }

    fn part1(&self, input: &str) -> Result<String> {
        let memory: Vec<i64> = input
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        Ok(run_with_params(&memory, 12, 2).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let memory: Vec<i64> = input
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        for noun in 0..100 {
            for verb in 0..100 {
                if run_with_params(&memory, noun, verb) == 19690720 {
                    return Ok((100 * noun + verb).to_string());
                }
            }
        }
        anyhow::bail!("No solution found")
    }
}

aoc_rust_common::aoc_test!(Day02);
