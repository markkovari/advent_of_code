use anyhow::Result;
use aoc_rust_common::Solution;
use std::collections::HashMap;

pub struct Day06;

impl Solution for Day06 {
    fn year(&self) -> u32 {
        2019
    }
    fn day(&self) -> u32 {
        6
    }

    fn part1(&self, input: &str) -> Result<String> {
        let orbits = parse(input);
        let mut total = 0;
        for start in orbits.keys() {
            total += count_orbits(&orbits, start);
        }
        Ok(total.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let orbits = parse(input);
        let path_you = get_path_to_com(&orbits, "YOU");
        let path_san = get_path_to_com(&orbits, "SAN");

        let mut common = "";
        for (i, p) in path_you.iter().enumerate() {
            if let Some(j) = path_san.iter().position(|x| x == p) {
                return Ok((i + j).to_string());
            }
        }
        anyhow::bail!("No common ancestor found")
    }
}

fn parse(input: &str) -> HashMap<&str, &str> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let parts: Vec<&str> = l.split(')').collect();
            (parts[1], parts[0])
        })
        .collect()
}

fn count_orbits(orbits: &HashMap<&str, &str>, start: &str) -> i32 {
    let mut current = start;
    let mut count = 0;
    while let Some(&next) = orbits.get(current) {
        count += 1;
        current = next;
    }
    count
}

fn get_path_to_com<'a>(orbits: &HashMap<&str, &'a str>, start: &str) -> Vec<&'a str> {
    let mut path = Vec::new();
    let mut current = start;
    while let Some(&next) = orbits.get(current) {
        path.push(next);
        current = next;
    }
    path
}

aoc_rust_common::aoc_test!(Day06);
