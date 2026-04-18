use anyhow::Result;
use aoc_rust_common::Solution;
use std::collections::HashSet;

pub struct Day15;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    col: i32,
    row: i32,
}

impl Coord {
    fn manhattan(&self, other: &Self) -> i32 {
        (self.row - other.row).abs() + (self.col - other.col).abs()
    }
}

fn parse(input: &str) -> Vec<[Coord; 2]> {
    input
        .lines()
        .map(|line| {
            let (s_str, b_str) = line.split_once(": ").unwrap();
            let (sx, sy) = s_str
                .strip_prefix("Sensor at ")
                .unwrap()
                .split_once(", ")
                .unwrap();
            let (bx, by) = b_str
                .strip_prefix("closest beacon is at ")
                .unwrap()
                .split_once(", ")
                .unwrap();
            [
                Coord {
                    col: sx.strip_prefix("x=").unwrap().parse().unwrap(),
                    row: sy.strip_prefix("y=").unwrap().parse().unwrap(),
                },
                Coord {
                    col: bx.strip_prefix("x=").unwrap().parse().unwrap(),
                    row: by.strip_prefix("y=").unwrap().parse().unwrap(),
                },
            ]
        })
        .collect()
}

fn get_row_ranges(row: i32, pairs: &[[Coord; 2]]) -> Vec<std::ops::RangeInclusive<i32>> {
    let mut ranges: Vec<std::ops::RangeInclusive<i32>> = pairs
        .iter()
        .filter_map(|p| {
            let radius = p[0].manhattan(&p[1]);
            let dist_to_row = (p[0].row - row).abs();
            if dist_to_row <= radius {
                let offset = radius - dist_to_row;
                Some(p[0].col - offset..=p[0].col + offset)
            } else {
                None
            }
        })
        .collect();
    ranges.sort_unstable_by_key(|r| *r.start());

    let mut merged = Vec::new();
    if !ranges.is_empty() {
        let mut curr = ranges[0].clone();
        for next in ranges.into_iter().skip(1) {
            if *next.start() <= *curr.end() + 1 {
                curr = *curr.start()..=*curr.end().max(next.end());
            } else {
                merged.push(curr);
                curr = next;
            }
        }
        merged.push(curr);
    }
    merged
}

impl Solution for Day15 {
    fn year(&self) -> u32 {
        2022
    }
    fn day(&self) -> u32 {
        15
    }

    fn part1(&self, input: &str) -> Result<String> {
        let pairs = parse(input);
        let row = 2_000_000;
        let ranges = get_row_ranges(row, &pairs);
        let count: i32 = ranges.iter().map(|r| r.end() - r.start() + 1).sum();
        let beacons: HashSet<i32> = pairs
            .iter()
            .filter(|p| p[1].row == row)
            .map(|p| p[1].col)
            .collect();
        Ok((count as usize - beacons.len()).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let pairs = parse(input);
        let max_coord = 4_000_000;
        for row in 0..=max_coord {
            let ranges = get_row_ranges(row, &pairs);
            if ranges.len() > 1 {
                let col = ranges[0].end() + 1;
                return Ok((col as i64 * 4_000_000 + row as i64).to_string());
            }
        }
        Ok((0).to_string())
    }
}
