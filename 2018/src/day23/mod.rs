use aoc_rust_common::Solution;
use regex::Regex;
use std::fmt::Display;

pub struct Day23;

#[derive(Debug)]
struct Bot {
    x: i64,
    y: i64,
    z: i64,
    r: i64,
}

impl Solution for Day23 {
    fn year(&self) -> u32 {
        2018
    }
    fn day(&self) -> u32 {
        23
    }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let bots: Vec<Bot> = input
            .lines()
            .map(|line| {
                let re = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
                let caps = re.captures(line).unwrap();
                Bot {
                    x: caps[1].parse().unwrap(),
                    y: caps[2].parse().unwrap(),
                    z: caps[3].parse().unwrap(),
                    r: caps[4].parse().unwrap(),
                }
            })
            .collect();

        let strongest = bots.iter().max_by_key(|b| b.r).unwrap();
        let count = bots
            .iter()
            .filter(|b| {
                (b.x - strongest.x).abs() + (b.y - strongest.y).abs() + (b.z - strongest.z).abs()
                    <= strongest.r
            })
            .count();
        Box::new(count)
    }

    fn part2(&self, _input: &str) -> Box<dyn Display> {
        // This is a complex problem (related to finding the point in the intersection of the most octahedrons).
        // The solution involves a search algorithm (like simulated annealing or a priority queue search over regions).
        // For now, returning a placeholder as the full implementation is very complex.
        Box::new("Not implemented")
    }
}
