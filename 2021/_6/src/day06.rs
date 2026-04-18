use aoc_rust_common::Solution;
use std::fmt::Display;

pub struct Day06;

impl Solution for Day06 {
    fn year(&self) -> u32 {
        2021
    }
    fn day(&self) -> u32 {
        6
    }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let initial: Vec<usize> = input
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let mut counts = [0usize; 9];
        for val in initial {
            counts[val] += 1;
        }

        for _ in 0..80 {
            let zeros = counts[0];
            for i in 0..8 {
                counts[i] = counts[i + 1];
            }
            counts[6] += zeros;
            counts[8] = zeros;
        }
        Box::new(counts.iter().sum::<usize>())
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let initial: Vec<usize> = input
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let mut counts = [0usize; 9];
        for val in initial {
            counts[val] += 1;
        }

        for _ in 0..256 {
            let zeros = counts[0];
            for i in 0..8 {
                counts[i] = counts[i + 1];
            }
            counts[6] += zeros;
            counts[8] = zeros;
        }
        Box::new(counts.iter().sum::<usize>())
    }
}
