use aoc_rust_common::Solution;
use std::fmt::Display;

pub struct Day14;

#[derive(Clone, Copy)]
enum Tile {
    Rock,
    Sand,
    Air,
}

impl Solution for Day14 {
    fn year(&self) -> u32 {
        2022
    }
    fn day(&self) -> u32 {
        14
    }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let mut grid = parse(input);
        let max_y = *grid.keys().map(|p| &p.1).max().unwrap();
        let mut sand_count = 0;
        'outer: loop {
            let mut sand = (500, 0);
            loop {
                if sand.1 > max_y {
                    break 'outer;
                }
                if !grid.contains_key(&(sand.0, sand.1 + 1)) {
                    sand.1 += 1;
                } else if !grid.contains_key(&(sand.0 - 1, sand.1 + 1)) {
                    sand.0 -= 1;
                    sand.1 += 1;
                } else if !grid.contains_key(&(sand.0 + 1, sand.1 + 1)) {
                    sand.0 += 1;
                    sand.1 += 1;
                } else {
                    grid.insert(sand, Tile::Sand);
                    sand_count += 1;
                    break;
                }
            }
        }
        Box::new(sand_count)
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let mut grid = parse(input);
        let max_y = *grid.keys().map(|p| &p.1).max().unwrap();
        let mut sand_count = 0;
        loop {
            let mut sand = (500, 0);
            loop {
                if sand.1 == max_y + 1 {
                    grid.insert(sand, Tile::Sand);
                    sand_count += 1;
                    break;
                }
                if !grid.contains_key(&(sand.0, sand.1 + 1)) {
                    sand.1 += 1;
                } else if !grid.contains_key(&(sand.0 - 1, sand.1 + 1)) {
                    sand.0 -= 1;
                    sand.1 += 1;
                } else if !grid.contains_key(&(sand.0 + 1, sand.1 + 1)) {
                    sand.0 += 1;
                    sand.1 += 1;
                } else {
                    grid.insert(sand, Tile::Sand);
                    sand_count += 1;
                    if sand == (500, 0) {
                        return Box::new(sand_count);
                    }
                    break;
                }
            }
        }
    }
}

fn parse(input: &str) -> std::collections::HashMap<(i32, i32), Tile> {
    let mut grid = std::collections::HashMap::new();
    for line in input.lines() {
        let coords: Vec<(i32, i32)> = line
            .split(" -> ")
            .map(|p| {
                let mut parts = p.split(',');
                (
                    parts.next().unwrap().parse().unwrap(),
                    parts.next().unwrap().parse().unwrap(),
                )
            })
            .collect();
        for i in 0..coords.len() - 1 {
            let (x1, y1) = coords[i];
            let (x2, y2) = coords[i + 1];
            for x in min(x1, x2)..=max(x1, x2) {
                for y in min(y1, y2)..=max(y1, y2) {
                    grid.insert((x, y), Tile::Rock);
                }
            }
        }
    }
    grid
}

fn min(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}
fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}
