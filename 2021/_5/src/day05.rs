use anyhow::Result;
use aoc_rust_common::Solution;
use std::collections::HashMap;

pub struct Day05;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn points(&self) -> Vec<Point> {
        let mut pts = Vec::new();
        let dx = (self.end.x - self.start.x).signum();
        let dy = (self.end.y - self.start.y).signum();
        let mut cur = self.start;
        while cur != self.end {
            pts.push(cur);
            cur.x += dx;
            cur.y += dy;
        }
        pts.push(self.end);
        pts
    }
}

fn parse(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(" -> ").collect();
            let start_coords: Vec<i32> = parts[0]
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            let end_coords: Vec<i32> = parts[1]
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            Line {
                start: Point {
                    x: start_coords[0],
                    y: start_coords[1],
                },
                end: Point {
                    x: end_coords[0],
                    y: end_coords[1],
                },
            }
        })
        .collect()
}

impl Solution for Day05 {
    fn year(&self) -> u32 {
        2021
    }
    fn day(&self) -> u32 {
        5
    }

    fn part1(&self, input: &str) -> Result<String> {
        let lines = parse(input);
        let mut counts = HashMap::new();
        for line in lines
            .iter()
            .filter(|l| l.start.x == l.end.x || l.start.y == l.end.y)
        {
            for pt in line.points() {
                *counts.entry(pt).or_insert(0) += 1;
            }
        }
        Ok((counts.values().filter(|&&v| v >= 2).count()).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let lines = parse(input);
        let mut counts = HashMap::new();
        for line in lines {
            for pt in line.points() {
                *counts.entry(pt).or_insert(0) += 1;
            }
        }
        Ok((counts.values().filter(|&&v| v >= 2).count()).to_string())
    }
}

aoc_rust_common::aoc_test!(Day05);
