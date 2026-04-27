use anyhow::Result;
use aoc_rust_common::Solution;

pub struct Day04;

impl Solution for Day04 {
    fn year(&self) -> u32 {
        2019
    }
    fn day(&self) -> u32 {
        4
    }

    fn part1(&self, input: &str) -> Result<String> {
        let range: Vec<i32> = input
            .trim()
            .split('-')
            .map(|s| s.parse().unwrap())
            .collect();
        let mut count = 0;
        for i in range[0]..=range[1] {
            if is_valid(i, false) {
                count += 1;
            }
        }
        Ok(count.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let range: Vec<i32> = input
            .trim()
            .split('-')
            .map(|s| s.parse().unwrap())
            .collect();
        let mut count = 0;
        for i in range[0]..=range[1] {
            if is_valid(i, true) {
                count += 1;
            }
        }
        Ok(count.to_string())
    }
}

fn is_valid(n: i32, restricted: bool) -> bool {
    let s = n.to_string();
    let digits: Vec<u32> = s.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let mut increasing = true;
    for i in 1..digits.len() {
        if digits[i] < digits[i - 1] {
            increasing = false;
            break;
        }
    }
    if !increasing {
        return false;
    }

    if restricted {
        let mut groups = Vec::new();
        let mut current_group = vec![digits[0]];
        for i in 1..digits.len() {
            if digits[i] == digits[i - 1] {
                current_group.push(digits[i]);
            } else {
                groups.push(current_group);
                current_group = vec![digits[i]];
            }
        }
        groups.push(current_group);
        groups.iter().any(|g| g.len() == 2)
    } else {
        for i in 1..digits.len() {
            if digits[i] == digits[i - 1] {
                return true;
            }
        }
        false
    }
}

aoc_rust_common::aoc_test!(Day04);
