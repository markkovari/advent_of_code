use aoc_rust_common::Solution;
use std::fmt::Display;
use std::str::FromStr;

pub struct Day02;

type Id = u64;

#[derive(Debug, Clone, Copy)]
struct Range { lower: Id, upper: Id }

impl FromStr for Range {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<Id> = s.split('-').map(|p| p.parse().unwrap()).collect();
        Ok(Range { lower: parts[0], upper: parts[1] })
    }
}

fn is_double(n: Id) -> bool {
    let s = n.to_string();
    let len = s.len();
    len % 2 == 0 && s[..len/2] == s[len/2..]
}

fn is_at_least_double(n: Id) -> bool {
    let s = n.to_string();
    let len = s.len();
    (1..len).filter(|d| len % d == 0 && len / d >= 2).any(|d| {
        let pattern = &s[..d];
        s.as_bytes().chunks(d).all(|c| c == pattern.as_bytes())
    })
}

impl Solution for Day02 {
    fn year(&self) -> u32 { 2025 }
    fn day(&self) -> u32 { 2 }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let ranges: Vec<Range> = input.trim().split(',').map(|s| s.parse().unwrap()).collect();
        let sum: Id = ranges.iter().flat_map(|r| (r.lower..=r.upper).filter(|&n| is_double(n))).sum();
        Box::new(sum)
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let ranges: Vec<Range> = input.trim().split(',').map(|s| s.parse().unwrap()).collect();
        let sum: Id = ranges.iter().flat_map(|r| (r.lower..=r.upper).filter(|&n| is_at_least_double(n))).sum();
        Box::new(sum)
    }
}
