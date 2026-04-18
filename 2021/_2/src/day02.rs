use aoc_rust_common::Solution;
use std::fmt::Display;

pub struct Day02;

impl Solution for Day02 {
    fn year(&self) -> u32 {
        2021
    }
    fn day(&self) -> u32 {
        2
    }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let (x, y, _) = input.lines().fold((0, 0, 0), |(px, py, aim), line| {
            let parts: Vec<&str> = line.split(' ').collect();
            let d = parts[0].chars().next().unwrap();
            let w: i32 = parts[1].parse().unwrap();
            match d {
                'f' => (px + w, py, aim),
                'u' => (px, py, aim - w),
                'd' => (px, py, aim + w),
                _ => (px, py, aim),
            }
        });
        Box::new(x * y)
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let (x, y, _) = input.lines().fold((0, 0, 0), |(px, py, aim), line| {
            let parts: Vec<&str> = line.split(' ').collect();
            let d = parts[0].chars().next().unwrap();
            let w: i32 = parts[1].parse().unwrap();
            match d {
                'f' => (px + w, py + aim * w, aim),
                'u' => (px, py, aim - w),
                'd' => (px, py, aim + w),
                _ => (px, py, aim),
            }
        });
        Box::new(x * y)
    }
}
