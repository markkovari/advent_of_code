use anyhow::Result;
use aoc_common::Solution;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub struct Day22;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Region {
    Rocky,
    Wet,
    Narrow,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Tool {
    Torch,
    ClimbingGear,
    Neither,
}

impl Solution for Day22 {
    fn year(&self) -> u32 {
        2018
    }
    fn day(&self) -> u32 {
        22
    }

    fn part1(&self, input: &str) -> Result<String> {
        let (depth, target) = parse(input);
        let mut erosion_levels = HashMap::new();
        let mut total_risk = 0;
        for y in 0..=target.1 {
            for x in 0..=target.0 {
                total_risk += get_erosion((x, y), depth, target, &mut erosion_levels) % 3;
            }
        }
        Ok((total_risk).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let (depth, target) = parse(input);
        let mut erosion_levels = HashMap::new();
        let mut dists = HashMap::new();
        let mut pq = BinaryHeap::new();

        dists.insert(((0, 0), Tool::Torch), 0);
        pq.push(Reverse((0, (0, 0), Tool::Torch)));

        while let Some(Reverse((time, pos, tool))) = pq.pop() {
            if dists.get(&(pos, tool)).map_or(false, |&t| t < time) {
                continue;
            }
            if pos == target && tool == Tool::Torch {
                return Ok((time).to_string());
            }

            for &next_tool in &[Tool::Torch, Tool::ClimbingGear, Tool::Neither] {
                if is_valid_tool(next_tool, get_type(pos, depth, target, &mut erosion_levels)) {
                    let next_time = time + if tool == next_tool { 0 } else { 7 };
                    if dists
                        .get(&(pos, next_tool))
                        .map_or(true, |&t| t > next_time)
                    {
                        dists.insert((pos, next_tool), next_time);
                    }
                }
            }

            for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let next_pos = (pos.0 + dx, pos.1 + dy);
                if next_pos.0 >= 0 && next_pos.1 >= 0 {
                    if is_valid_tool(tool, get_type(next_pos, depth, target, &mut erosion_levels)) {
                        let next_time = time + 1;
                        if dists
                            .get(&(next_pos, tool))
                            .map_or(true, |&t| t > next_time)
                        {
                            dists.insert((next_pos, tool), next_time);
                            pq.push(Reverse((next_time, next_pos, tool)));
                        }
                    }
                }
            }
        }
        Ok(("No path found").to_string())
    }
}

fn parse(input: &str) -> (i32, (i32, i32)) {
    let mut lines = input.lines();
    let depth: i32 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let target_coords: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    (depth, (target_coords[0], target_coords[1]))
}

fn get_erosion(
    pos: (i32, i32),
    depth: i32,
    target: (i32, i32),
    memo: &mut HashMap<(i32, i32), i32>,
) -> i32 {
    if let Some(&erosion) = memo.get(&pos) {
        return erosion;
    }
    let geo_index = match pos {
        (0, 0) => 0,
        p if p == target => 0,
        (x, 0) => x * 16807,
        (0, y) => y * 48271,
        (x, y) => {
            get_erosion((x - 1, y), depth, target, memo)
                * get_erosion((x, y - 1), depth, target, memo)
        }
    };
    let erosion = (geo_index + depth) % 20183;
    memo.insert(pos, erosion);
    erosion
}

fn get_type(
    pos: (i32, i32),
    depth: i32,
    target: (i32, i32),
    memo: &mut HashMap<(i32, i32), i32>,
) -> Region {
    match get_erosion(pos, depth, target, memo) % 3 {
        0 => Region::Rocky,
        1 => Region::Wet,
        2 => Region::Narrow,
        _ => unreachable!(),
    }
}

fn is_valid_tool(tool: Tool, region: Region) -> bool {
    match region {
        Region::Rocky => tool == Tool::ClimbingGear || tool == Tool::Torch,
        Region::Wet => tool == Tool::ClimbingGear || tool == Tool::Neither,
        Region::Narrow => tool == Tool::Torch || tool == Tool::Neither,
    }
}

aoc_common::aoc_test!(Day22);
