use anyhow::Result;
use aoc_rust_common::Solution;

pub struct Day01;

fn required_fuel(mass: i64) -> i64 {
    let fuel = mass / 3 - 2;
    if fuel < 0 {
        0
    } else {
        fuel
    }
}

fn required_fuel_with_tank(mass: i64) -> i64 {
    let fuel = mass / 3 - 2;
    if fuel < 0 {
        0
    } else {
        fuel + required_fuel_with_tank(fuel)
    }
}

impl Solution for Day01 {
    fn year(&self) -> u32 {
        2019
    }
    fn day(&self) -> u32 {
        1
    }

    fn part1(&self, input: &str) -> Result<String> {
        let res: i64 = input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.parse::<i64>().unwrap())
            .map(required_fuel)
            .sum();
        Ok(res.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let res: i64 = input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.parse::<i64>().unwrap())
            .map(required_fuel_with_tank)
            .sum();
        Ok(res.to_string())
    }
}

aoc_rust_common::aoc_test!(Day01);
