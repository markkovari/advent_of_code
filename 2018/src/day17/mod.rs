use aoc_rust_common::Solution;
use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::fmt::Display;

pub struct Day17;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn parse(input: &str) -> (HashSet<Point>, i32, i32) {
    let mut clay = HashSet::new();
    let re = Regex::new(r"(x|y)=(\d+), (x|y)=(\d+)..(\d+)").unwrap();
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let a = caps[2].parse().unwrap();
        let b1: i32 = caps[4].parse().unwrap();
        let b2: i32 = caps[5].parse().unwrap();

        if &caps[1] == "x" {
            min_y = min(min_y, b1);
            max_y = max(max_y, b2);
            for y in b1..=b2 {
                clay.insert(Point { x: a, y });
            }
        } else {
            min_y = min(min_y, a);
            max_y = max(max_y, a);
            for x in b1..=b2 {
                clay.insert(Point { x, y: a });
            }
        }
    }
    (clay, min_y, max_y)
}

fn solve(input: &str) -> (usize, usize) {
    let (clay, min_y, max_y) = parse(input);
    let mut flowing_water = HashSet::new();
    let mut still_water = HashSet::new();
    let mut stack = vec![Point { x: 500, y: 0 }];

    while let Some(point) = stack.pop() {
        if point.y > max_y {
            continue;
        }

        let mut current = point;
        while current.y <= max_y && !clay.contains(&current) {
            flowing_water.insert(current);
            current.y += 1;
        }

        if current.y > max_y {
            continue;
        }

        current.y -= 1; // Step back up to solid ground

        loop {
            let mut left = current;
            let mut right = current;
            let mut wall_left = false;
            let mut wall_right = false;

            while !clay.contains(&Point {
                x: left.x - 1,
                y: left.y,
            }) {
                left.x -= 1;
                let down = Point {
                    x: left.x,
                    y: left.y + 1,
                };
                if !clay.contains(&down) && !still_water.contains(&down) {
                    stack.push(left);
                    break;
                }
            }
            if clay.contains(&Point {
                x: left.x - 1,
                y: left.y,
            }) {
                wall_left = true;
            }

            while !clay.contains(&Point {
                x: right.x + 1,
                y: right.y,
            }) {
                right.x += 1;
                let down = Point {
                    x: right.x,
                    y: right.y + 1,
                };
                if !clay.contains(&down) && !still_water.contains(&down) {
                    stack.push(right);
                    break;
                }
            }
            if clay.contains(&Point {
                x: right.x + 1,
                y: right.y,
            }) {
                wall_right = true;
            }

            for x in left.x..=right.x {
                if wall_left && wall_right {
                    still_water.insert(Point { x, y: current.y });
                } else {
                    flowing_water.insert(Point { x, y: current.y });
                }
            }

            if !(wall_left && wall_right) {
                break;
            }
            current.y -= 1;
        }
    }

    let total_water = flowing_water
        .union(&still_water)
        .filter(|p| p.y >= min_y && p.y <= max_y)
        .count();
    let still_water_count = still_water.len();
    (total_water, still_water_count)
}

impl Solution for Day17 {
    fn year(&self) -> u32 {
        2018
    }
    fn day(&self) -> u32 {
        17
    }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let (total_water, _) = solve(input);
        Box::new(total_water)
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let (_, still_water) = solve(input);
        Box::new(still_water)
    }
}
