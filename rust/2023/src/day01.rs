use anyhow::Result;
use aoc_rust_common::Solution;

pub struct Day01;

impl Solution for Day01 {
    fn year(&self) -> u32 {
        2023
    }
    fn day(&self) -> u32 {
        1
    }

    fn part1(&self, input: &str) -> Result<String> {
        let result: u32 = input
            .lines()
            .map(|line| {
                let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
                digits.first().unwrap() * 10 + digits.last().unwrap()
            })
            .sum();
        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let result: u32 = input
            .lines()
            .map(|line| {
                let digits = get_mixed_up_digits(line);
                digits.first().unwrap() * 10 + digits.last().unwrap()
            })
            .sum();
        Ok(result.to_string())
    }
}

fn get_mixed_up_digits(line: &str) -> Vec<u32> {
    let mut digits = Vec::new();
    for i in 0..line.len() {
        let slice = &line[i..];
        if slice.starts_with("one") {
            digits.push(1);
        } else if slice.starts_with("two") {
            digits.push(2);
        } else if slice.starts_with("three") {
            digits.push(3);
        } else if slice.starts_with("four") {
            digits.push(4);
        } else if slice.starts_with("five") {
            digits.push(5);
        } else if slice.starts_with("six") {
            digits.push(6);
        } else if slice.starts_with("seven") {
            digits.push(7);
        } else if slice.starts_with("eight") {
            digits.push(8);
        } else if slice.starts_with("nine") {
            digits.push(9);
        } else if let Some(d) = slice.chars().next().unwrap().to_digit(10) {
            digits.push(d);
        }
    }
    digits
}

aoc_rust_common::aoc_test!(Day01, "54573", "54591");
