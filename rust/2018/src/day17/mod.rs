use anyhow::Result;
use aoc_common::Solution;
use regex::Regex;
use std::cmp::{max, min};

pub struct Day17;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Sand,
    Clay,
    Flowing,
    Still,
}

fn parse(input: &str) -> (Vec<Vec<Tile>>, i32, i32, i32, i32) {
    let mut clay_points = Vec::new();
    let re = Regex::new(r"(x|y)=(\d+), (x|y)=(\d+)..(\d+)").unwrap();
    let (mut min_x, mut max_x) = (i32::MAX, i32::MIN);
    let (mut min_y, mut max_y) = (i32::MAX, i32::MIN);

    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let a = caps[2].parse().unwrap();
        let b1: i32 = caps[4].parse().unwrap();
        let b2: i32 = caps[5].parse().unwrap();

        if &caps[1] == "x" {
            let x = a;
            min_x = min(min_x, x);
            max_x = max(max_x, x);
            min_y = min(min_y, b1);
            max_y = max(max_y, b2);
            for y in b1..=b2 {
                clay_points.push((x, y));
            }
        } else {
            let y = a;
            min_y = min(min_y, y);
            max_y = max(max_y, y);
            min_x = min(min_x, b1);
            max_x = max(max_x, b2);
            for x in b1..=b2 {
                clay_points.push((x, y));
            }
        }
    }

    // Add padding to x to allow water to flow around clay
    min_x -= 1;
    max_x += 1;

    let width = (max_x - min_x + 1) as usize;
    let height = (max_y + 1) as usize;
    let mut grid = vec![vec![Tile::Sand; width]; height];

    for (x, y) in clay_points {
        grid[y as usize][(x - min_x) as usize] = Tile::Clay;
    }

    (grid, min_x, max_x, min_y, max_y)
}

fn solve(input: &str) -> (usize, usize) {
    let (mut grid, min_x, _, min_y, max_y) = parse(input);
    let mut stack = vec![(500i32, 0i32)];

    while let Some((sx, sy)) = stack.pop() {
        let mut y = sy;
        let x = sx;

        // Flow down
        while y <= max_y && grid[y as usize][(x - min_x) as usize] == Tile::Sand {
            grid[y as usize][(x - min_x) as usize] = Tile::Flowing;
            y += 1;
        }

        if y > max_y || grid[y as usize][(x - min_x) as usize] == Tile::Flowing {
            continue;
        }

        // We hit clay or still water, move back up and fill levels
        y -= 1;
        while y >= sy {
            let mut left_x = x;
            let mut left_bound = false;
            while grid[y as usize][(left_x - min_x) as usize] != Tile::Clay {
                let below = grid[(y + 1) as usize][(left_x - min_x) as usize];
                if below == Tile::Sand || below == Tile::Flowing {
                    break;
                }
                left_x -= 1;
            }
            if grid[y as usize][(left_x - min_x) as usize] == Tile::Clay {
                left_bound = true;
                left_x += 1;
            }

            let mut right_x = x;
            let mut right_bound = false;
            while grid[y as usize][(right_x - min_x) as usize] != Tile::Clay {
                let below = grid[(y + 1) as usize][(right_x - min_x) as usize];
                if below == Tile::Sand || below == Tile::Flowing {
                    break;
                }
                right_x += 1;
            }
            if grid[y as usize][(right_x - min_x) as usize] == Tile::Clay {
                right_bound = true;
                right_x -= 1;
            }

            if left_bound && right_bound {
                for fill_x in left_x..=right_x {
                    grid[y as usize][(fill_x - min_x) as usize] = Tile::Still;
                }
            } else {
                for fill_x in left_x..=right_x {
                    grid[y as usize][(fill_x - min_x) as usize] = Tile::Flowing;
                }
                if !left_bound {
                    stack.push((left_x, y));
                }
                if !right_bound {
                    stack.push((right_x, y));
                }
                break;
            }
            y -= 1;
        }
    }

    let mut total_water = 0;
    let mut still_water = 0;
    for y in min_y as usize..=max_y as usize {
        for x in 0..grid[y].len() {
            match grid[y][x] {
                Tile::Flowing => total_water += 1,
                Tile::Still => {
                    total_water += 1;
                    still_water += 1;
                }
                _ => {}
            }
        }
    }

    (total_water, still_water)
}

impl Solution for Day17 {
    fn year(&self) -> u32 {
        2018
    }
    fn day(&self) -> u32 {
        17
    }

    fn part1(&self, input: &str) -> Result<String> {
        let (total_water, _) = solve(input);
        Ok((total_water).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let (_, still_water) = solve(input);
        Ok((still_water).to_string())
    }
}

aoc_common::aoc_test!(Day17, 162, 144, slow);
