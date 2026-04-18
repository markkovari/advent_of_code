use anyhow::Result;
use aoc_rust_common::Solution;
use iter_tools::Itertools;

pub struct Day24;

fn find_qe(packages: &[u64], groups: usize) -> u64 {
    let total_weight: u64 = packages.iter().sum();
    let group_weight = total_weight / groups as u64;

    for i in 1..packages.len() {
        let mut best_qe = u64::MAX;
        let mut found = false;

        for c in packages.iter().combinations(i) {
            if c.iter().copied().sum::<u64>() == group_weight {
                found = true;
                best_qe = best_qe.min(c.iter().copied().product::<u64>());
            }
        }
        if found {
            return best_qe;
        }
    }
    0
}

impl Solution for Day24 {
    fn year(&self) -> u32 {
        2015
    }
    fn day(&self) -> u32 {
        24
    }

    fn part1(&self, input: &str) -> Result<String> {
        let packages: Vec<u64> = input.lines().map(|l| l.parse().unwrap()).collect();
        Ok((find_qe(&packages, 3)).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let packages: Vec<u64> = input.lines().map(|l| l.parse().unwrap()).collect();
        Ok((find_qe(&packages, 4)).to_string())
    }
}
