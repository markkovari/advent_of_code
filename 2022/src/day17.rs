use anyhow::Result;
use aoc_rust_common::Solution;
use std::collections::{HashMap, HashSet};

pub struct Day17;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Rock {
    Horizontal,
    Plus,
    Corner,
    Vertical,
    Square,
}

impl Rock {
    fn get_offsets(&self) -> Vec<(i32, i32)> {
        match self {
            Rock::Horizontal => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Rock::Plus => vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            Rock::Corner => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            Rock::Vertical => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            Rock::Square => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        }
    }
}

const ROCKS: [Rock; 5] = [
    Rock::Horizontal,
    Rock::Plus,
    Rock::Corner,
    Rock::Vertical,
    Rock::Square,
];

impl Solution for Day17 {
    fn year(&self) -> u32 {
        2022
    }
    fn day(&self) -> u32 {
        17
    }

    fn part1(&self, input: &str) -> Result<String> {
        Ok(solve(input, 2022).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        Ok(solve(input, 1_000_000_000_000).to_string())
    }
}

fn solve(input: &str, count: u64) -> u64 {
    let jets: Vec<i32> = input
        .trim()
        .chars()
        .map(|c| if c == '>' { 1 } else { -1 })
        .collect();
    let mut grid = HashSet::new();
    for x in 0..7 {
        grid.insert((x, 0));
    }

    let mut height = 0;
    let mut jet_idx = 0;
    let mut seen = HashMap::new();

    for i in 0..count {
        let rock_idx = (i % 5) as usize;
        let rock = ROCKS[rock_idx];
        let state = (rock_idx, jet_idx, get_profile(&grid, height));

        if let Some(&(prev_i, prev_h)) = seen.get(&state) {
            let cycle_len = i - prev_i;
            let cycle_h = height - prev_h;
            let remaining = count - i;
            let cycles = remaining / cycle_len;
            let remaining_after_cycles = remaining % cycle_len;

            return height as u64
                + (cycles * (cycle_h as u64))
                + (solve_remainder(
                    &grid,
                    &jets,
                    jet_idx,
                    rock_idx as u64,
                    remaining_after_cycles,
                    height,
                ) as u64
                    - height as u64);
        }
        seen.insert(state, (i, height));

        let mut pos = (2, height + 4);
        loop {
            let jet = jets[jet_idx];
            jet_idx = (jet_idx + 1) % jets.len();
            if can_move(&grid, &rock, pos.0 + jet, pos.1) {
                pos.0 += jet;
            }
            if can_move(&grid, &rock, pos.0, pos.1 - 1) {
                pos.1 -= 1;
            } else {
                for (dx, dy) in rock.get_offsets() {
                    grid.insert((pos.0 + dx, pos.1 + dy));
                    height = height.max(pos.1 + dy);
                }
                break;
            }
        }
    }
    height as u64
}

fn get_profile(grid: &HashSet<(i32, i32)>, height: i32) -> Vec<i32> {
    (0..7)
        .map(|x| {
            (0..=height)
                .rev()
                .find(|&y| grid.contains(&(x, y)))
                .map_or(height, |y| height - y)
        })
        .collect()
}

fn can_move(grid: &HashSet<(i32, i32)>, rock: &Rock, x: i32, y: i32) -> bool {
    if x < 0
        || x + match rock {
            Rock::Horizontal => 3,
            Rock::Plus => 2,
            Rock::Corner => 2,
            Rock::Vertical => 0,
            Rock::Square => 1,
        } >= 7
    {
        return false;
    }

    rock.get_offsets().iter().all(|&(dx, dy)| {
        let nx = x + dx;
        let ny = y + dy;
        ny > 0 && !grid.contains(&(nx, ny))
    })
}

fn solve_remainder(
    grid: &HashSet<(i32, i32)>,
    jets: &[i32],
    mut jet_idx: usize,
    rock_offset: u64,
    count: u64,
    mut height: i32,
) -> i32 {
    let mut grid = grid.clone();
    for i in 0..count {
        let rock = ROCKS[((rock_offset + i) % 5) as usize];
        let mut pos = (2, height + 4);
        loop {
            let jet = jets[jet_idx];
            jet_idx = (jet_idx + 1) % jets.len();
            if can_move(&grid, &rock, pos.0 + jet, pos.1) {
                pos.0 += jet;
            }
            if can_move(&grid, &rock, pos.0, pos.1 - 1) {
                pos.1 -= 1;
            } else {
                for (dx, dy) in rock.get_offsets() {
                    grid.insert((pos.0 + dx, pos.1 + dy));
                    height = height.max(pos.1 + dy);
                }
                break;
            }
        }
    }
    height
}
