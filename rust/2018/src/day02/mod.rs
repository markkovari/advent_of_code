use anyhow::Result;
use aoc_common::Solution;
use std::collections::HashMap;

pub struct Day02;

fn get_two_closest(of_ids: &[String]) -> (String, String) {
    let mut closest = (String::new(), String::new());
    let mut min = usize::max_value();
    for (i, id) in of_ids.iter().enumerate() {
        for (j, other) in of_ids.iter().enumerate() {
            if i == j {
                continue;
            }
            let diff = id
                .chars()
                .zip(other.chars())
                .filter(|(c1, c2)| c1 != c2)
                .count();
            if diff < min {
                min = diff;
                closest = (id.clone(), other.clone());
            }
        }
    }
    closest
}

fn get_common_chars(a: &str, b: &str) -> String {
    a.chars()
        .zip(b.chars())
        .filter(|(c1, c2)| c1 == c2)
        .map(|(c, _)| c)
        .collect()
}

impl Solution for Day02 {
    fn year(&self) -> u32 {
        2018
    }
    fn day(&self) -> u32 {
        2
    }

    fn part1(&self, input: &str) -> Result<String> {
        let (twos, threes) = input.lines().fold((0, 0), |(mut twos, mut threes), line| {
            let mut counts = HashMap::new();
            for c in line.chars() {
                *counts.entry(c).or_insert(0) += 1;
            }
            if counts.values().any(|&v| v == 2) {
                twos += 1;
            }
            if counts.values().any(|&v| v == 3) {
                threes += 1;
            }
            (twos, threes)
        });
        Ok((twos * threes).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let ids: Vec<String> = input.lines().map(|s| s.to_string()).collect();
        let (a, b) = get_two_closest(&ids);
        Ok((get_common_chars(&a, &b)).to_string())
    }
}

aoc_common::aoc_test!(Day02);
