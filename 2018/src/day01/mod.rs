use aoc_rust_common::Solution;
use std::collections::HashSet;
use std::fmt::Display;

pub struct Day01;

impl Solution for Day01 {
    fn year(&self) -> u32 {
        2018
    }
    fn day(&self) -> u32 {
        1
    }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let sum: i32 = input.lines().map(|s| s.parse::<i32>().unwrap()).sum();
        Box::new(sum)
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let numbers: Vec<i32> = input.lines().map(|s| s.parse::<i32>().unwrap()).collect();
        let mut visited = HashSet::new();
        let mut current = 0;
        visited.insert(current);
        loop {
            for i in numbers.iter() {
                current += i;
                if visited.contains(&current) {
                    return Box::new(current);
                }
                visited.insert(current);
            }
        }
    }
}
