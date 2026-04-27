use crate::utils::intcode::{growing_memory, VM};
use anyhow::Result;
use aoc_rust_common::{bfs, Direction, Point, Solution};
use std::collections::HashMap;

pub struct Day15;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Open,
    OxygenSystem,
}

impl Solution for Day15 {
    fn year(&self) -> u32 {
        2019
    }
    fn day(&self) -> u32 {
        15
    }

    fn part1(&self, input: &str) -> Result<String> {
        let (map, _) = explore_map(input)?;
        let start = Point::new(0, 0);
        let dist = bfs(
            start,
            |p| {
                p.cardinal_neighbors()
                    .iter()
                    .cloned()
                    .filter(|n| map.get(n) == Some(&Tile::Open) || map.get(n) == Some(&Tile::OxygenSystem))
                    .collect()
            },
            |p| map.get(p) == Some(&Tile::OxygenSystem),
        );
        Ok(dist.unwrap().to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let (map, oxygen_pos) = explore_map(input)?;
        let mut max_dist = 0;

        // Find max distance from oxygen_pos to any open tile using BFS
        let mut visited = HashMap::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((oxygen_pos, 0));
        visited.insert(oxygen_pos, 0);

        while let Some((curr, dist)) = queue.pop_front() {
            max_dist = max_dist.max(dist);
            for neighbor in curr.cardinal_neighbors() {
                if !visited.contains_key(&neighbor) && map.get(&neighbor) == Some(&Tile::Open) {
                    visited.insert(neighbor, dist + 1);
                    queue.push_back((neighbor, dist + 1));
                }
            }
        }

        Ok(max_dist.to_string())
    }
}

fn explore_map(input: &str) -> Result<(HashMap<Point, Tile>, Point)> {
    let memory: Vec<i64> = input
        .trim()
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();
    let mut vm = VM::new(growing_memory(memory));
    let mut map = HashMap::new();
    let mut pos = Point::new(0, 0);
    let mut oxygen_pos = Point::new(0, 0);
    map.insert(pos, Tile::Open);

    // Depth-first search to explore the entire maze
    fn dfs(
        vm: &mut VM<crate::utils::intcode::GrowingMemory>,
        pos: Point,
        map: &mut HashMap<Point, Tile>,
        oxygen_pos: &mut Point,
    ) -> Result<()> {
        for d in [Direction::North, Direction::South, Direction::West, Direction::East] {
            let next_pos = pos + d.delta();
            if map.contains_key(&next_pos) {
                continue;
            }

            let cmd = match d {
                Direction::North => 1,
                Direction::South => 2,
                Direction::West => 3,
                Direction::East => 4,
                _ => unreachable!(),
            };

            vm.registers.pending_in = Some(cmd);
            while vm.registers.pending_out.is_none() {
                vm.run_all_async(|_| Ok(()))?;
            }
            let status = vm.registers.pending_out.take().unwrap();

            match status {
                0 => {
                    map.insert(next_pos, Tile::Wall);
                }
                1 => {
                    map.insert(next_pos, Tile::Open);
                    dfs(vm, next_pos, map, oxygen_pos)?;
                    // Move back
                    let back_cmd = match d {
                        Direction::North => 2,
                        Direction::South => 1,
                        Direction::West => 4,
                        Direction::East => 3,
                        _ => unreachable!(),
                    };
                    vm.registers.pending_in = Some(back_cmd);
                    while vm.registers.pending_out.is_none() {
                        vm.run_all_async(|_| Ok(()))?;
                    }
                    vm.registers.pending_out.take();
                }
                2 => {
                    map.insert(next_pos, Tile::OxygenSystem);
                    *oxygen_pos = next_pos;
                    dfs(vm, next_pos, map, oxygen_pos)?;
                    // Move back
                    let back_cmd = match d {
                        Direction::North => 2,
                        Direction::South => 1,
                        Direction::West => 4,
                        Direction::East => 3,
                        _ => unreachable!(),
                    };
                    vm.registers.pending_in = Some(back_cmd);
                    while vm.registers.pending_out.is_none() {
                        vm.run_all_async(|_| Ok(()))?;
                    }
                    vm.registers.pending_out.take();
                }
                _ => panic!("Invalid status"),
            }
        }
        Ok(())
    }

    dfs(&mut vm, pos, &mut map, &mut oxygen_pos)?;
    Ok((map, oxygen_pos))
}

aoc_rust_common::aoc_test!(Day15);
