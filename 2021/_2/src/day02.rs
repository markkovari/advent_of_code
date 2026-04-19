use anyhow::Result;
use aoc_rust_common::Solution;

pub struct Day02;

impl Solution for Day02 {
    fn year(&self) -> u32 {
        2021
    }
    fn day(&self) -> u32 {
        2
    }

    fn part1(&self, input: &str) -> Result<String> {
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
        Ok((x * y).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
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
        Ok((x * y).to_string())
    }
}

aoc_rust_common::aoc_test!(Day02);
