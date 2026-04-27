use anyhow::Result;
use aoc_rust_common::Solution;
use lazy_static::lazy_static;
use regex::Regex;

pub struct Day25;

fn get_code(row: u64, col: u64) -> u64 {
    let n = row + col - 1;
    let index = (n * (n - 1) / 2) + col;
    let mut code = 20151125;
    for _ in 1..index {
        code = (code * 252533) % 33554393;
    }
    code
}

impl Solution for Day25 {
    fn year(&self) -> u32 {
        2015
    }
    fn day(&self) -> u32 {
        25
    }

    fn part1(&self, input: &str) -> Result<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"row (\d+), column (\d+)").unwrap();
        }
        let caps = RE.captures(input).unwrap();
        let row = caps[1].parse::<u64>().unwrap();
        let col = caps[2].parse::<u64>().unwrap();
        Ok((get_code(row, col)).to_string())
    }

    fn part2(&self, _input: &str) -> Result<String> {
        Ok(("There is no part 2 for Day 25!").to_string())
    }
}

aoc_rust_common::aoc_test!(Day25);
