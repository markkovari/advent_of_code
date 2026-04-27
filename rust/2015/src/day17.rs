use anyhow::Result;
use aoc_rust_common::Solution;
use iter_tools::Itertools;
use rayon::prelude::*;

pub struct Day17;

fn read_containers(content: &str) -> Vec<u64> {
    content
        .lines()
        .filter_map(|line| line.parse::<u64>().ok())
        .collect()
}

fn get_combinations(containers: Vec<u64>, liters: u64) -> Vec<Vec<u64>> {
    (1..=containers.len())
        .into_par_iter()
        .flat_map(|i| {
            containers
                .iter()
                .combinations(i)
                .par_bridge()
                .filter_map(move |combination| {
                    if combination.iter().map(|e| **e).sum::<u64>() == liters {
                        Some(combination.into_iter().copied().collect())
                    } else {
                        None
                    }
                })
                .collect::<Vec<Vec<u64>>>()
        })
        .collect()
}

impl Solution for Day17 {
    fn year(&self) -> u32 {
        2015
    }
    fn day(&self) -> u32 {
        17
    }

    fn part1(&self, input: &str) -> Result<String> {
        let containers = read_containers(input);
        let liters = if containers.len() < 10 { 25 } else { 150 };
        Ok((get_combinations(containers, liters).len() as i64).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let containers = read_containers(input);
        let liters = if containers.len() < 10 { 25 } else { 150 };
        let combinations = get_combinations(containers, liters);
        let min_len = combinations.iter().map(|c| c.len()).min().unwrap_or(0);
        Ok((combinations.iter().filter(|c| c.len() == min_len).count() as i64).to_string())
    }
}

aoc_rust_common::aoc_test!(Day17);
