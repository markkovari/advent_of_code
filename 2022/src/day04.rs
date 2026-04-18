use anyhow::Result;
use aoc_rust_common::Solution;
use std::str::FromStr;

pub struct Day04;

#[derive(Debug, Clone, Copy)]
struct Range {
    from: i32,
    until: i32,
}

impl FromStr for Range {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<i32> = s.split('-').map(|p| p.parse().unwrap()).collect();
        Ok(Range {
            from: parts[0],
            until: parts[1],
        })
    }
}

impl Solution for Day04 {
    fn year(&self) -> u32 {
        2022
    }
    fn day(&self) -> u32 {
        4
    }

    fn part1(&self, input: &str) -> Result<String> {
        let count = input
            .lines()
            .filter(|line| {
                let (r1, r2) = parse_ranges(line);
                (r1.from <= r2.from && r1.until >= r2.until)
                    || (r2.from <= r1.from && r2.until >= r1.until)
            })
            .count();
        Ok((count).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let count = input
            .lines()
            .filter(|line| {
                let (r1, r2) = parse_ranges(line);
                r1.from <= r2.until && r1.until >= r2.from
            })
            .count();
        Ok((count).to_string())
    }
}

fn parse_ranges(line: &str) -> (Range, Range) {
    let mut parts = line.split(',');
    (
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
    )
}
