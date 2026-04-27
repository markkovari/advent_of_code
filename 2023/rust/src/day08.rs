use anyhow::Result;
use aoc_rust_common::Solution;
use std::collections::HashMap;

pub struct Day08;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    (a * b) / gcd(a, b)
}

impl Solution for Day08 {
    fn year(&self) -> u32 {
        2023
    }
    fn day(&self) -> u32 {
        8
    }

    fn part1(&self, input: &str) -> Result<String> {
        let mut sections = input.split("\n\n");
        let instructions = sections.next().unwrap();
        let network_raw = sections.next().unwrap();

        let mut network = HashMap::new();
        for line in network_raw.lines() {
            let parts: Vec<&str> = line.split(" = ").collect();
            let from = parts[0];
            let to_parts: Vec<&str> = parts[1]
                .trim_matches(|c| c == '(' || c == ')')
                .split(", ")
                .collect();
            network.insert(from, (to_parts[0], to_parts[1]));
        }

        let mut current = "AAA";
        let mut steps = 0;
        let instr_chars: Vec<char> = instructions.chars().collect();

        while current != "ZZZ" {
            let instr = instr_chars[steps % instr_chars.len()];
            let (left, right) = network.get(current).unwrap();
            current = if instr == 'L' { left } else { right };
            steps += 1;
        }

        Ok(steps.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut sections = input.split("\n\n");
        let instructions = sections.next().unwrap();
        let network_raw = sections.next().unwrap();

        let mut network = HashMap::new();
        for line in network_raw.lines() {
            let parts: Vec<&str> = line.split(" = ").collect();
            let from = parts[0];
            let to_parts: Vec<&str> = parts[1]
                .trim_matches(|c| c == '(' || c == ')')
                .split(", ")
                .collect();
            network.insert(from, (to_parts[0], to_parts[1]));
        }

        let starts: Vec<&str> = network
            .keys()
            .filter(|&&k| k.ends_with('A'))
            .cloned()
            .collect();
        let instr_chars: Vec<char> = instructions.chars().collect();

        let mut cycle_lengths = Vec::new();
        for start in starts {
            let mut current = start;
            let mut steps = 0;
            while !current.ends_with('Z') {
                let instr = instr_chars[steps % instr_chars.len()];
                let (left, right) = network.get(current).unwrap();
                current = if instr == 'L' { left } else { right };
                steps += 1;
            }
            cycle_lengths.push(steps as u64);
        }

        let result = cycle_lengths.into_iter().reduce(lcm).unwrap();
        Ok(result.to_string())
    }
}

aoc_rust_common::aoc_test!(Day08, "19241", "9606140307013");
