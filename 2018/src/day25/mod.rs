use anyhow::Result;
use aoc_rust_common::Solution;
use std::collections::HashSet;

pub struct Day25;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    t: i32,
}

impl Point {
    fn dist(&self, other: &Point) -> i32 {
        (self.x - other.x).abs()
            + (self.y - other.y).abs()
            + (self.z - other.z).abs()
            + (self.t - other.t).abs()
    }
}

impl Solution for Day25 {
    fn year(&self) -> u32 {
        2018
    }
    fn day(&self) -> u32 {
        25
    }

    fn part1(&self, input: &str) -> Result<String> {
        let points: Vec<Point> = input
            .lines()
            .map(|line| {
                let coords: Vec<i32> = line.split(',').map(|s| s.parse().unwrap()).collect();
                Point {
                    x: coords[0],
                    y: coords[1],
                    z: coords[2],
                    t: coords[3],
                }
            })
            .collect();

        let mut constellations = 0;
        let mut visited = HashSet::new();

        for i in 0..points.len() {
            if !visited.contains(&i) {
                constellations += 1;
                let mut stack = vec![i];
                visited.insert(i);

                while let Some(current_idx) = stack.pop() {
                    for j in 0..points.len() {
                        if !visited.contains(&j) && points[current_idx].dist(&points[j]) <= 3 {
                            visited.insert(j);
                            stack.push(j);
                        }
                    }
                }
            }
        }
        Ok((constellations).to_string())
    }

    fn part2(&self, _input: &str) -> Result<String> {
        Ok(("There is no part 2 for Day 25!").to_string())
    }
}
