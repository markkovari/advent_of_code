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
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<i32> = s
            .split('-')
            .map(|p| {
                p.parse()
                    .map_err(|e| anyhow::anyhow!("failed to parse range part: {}", e))
            })
            .collect::<Result<Vec<i32>>>()?;
        if parts.len() < 2 {
            return Err(anyhow::anyhow!("invalid range: {}", s));
        }
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
        let mut count = 0;
        for line in input.lines() {
            let (r1, r2) = parse_ranges(line)?;
            if (r1.from <= r2.from && r1.until >= r2.until)
                || (r2.from <= r1.from && r2.until >= r1.until)
            {
                count += 1;
            }
        }
        Ok((count).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut count = 0;
        for line in input.lines() {
            let (r1, r2) = parse_ranges(line)?;
            if r1.from <= r2.until && r1.until >= r2.from {
                count += 1;
            }
        }
        Ok((count).to_string())
    }
}

fn parse_ranges(line: &str) -> Result<(Range, Range)> {
    let mut parts = line.split(',');
    let r1 = parts
        .next()
        .ok_or_else(|| anyhow::anyhow!("missing first range"))?
        .parse()?;
    let r2 = parts
        .next()
        .ok_or_else(|| anyhow::anyhow!("missing second range"))?
        .parse()?;
    Ok((r1, r2))
}

aoc_rust_common::aoc_test!(Day04);
