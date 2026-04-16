use aoc_rust_common::Solution;
use std::fmt::Display;
use std::collections::HashMap;

pub struct Day17;

#[derive(Clone, Copy, PartialEq, Eq)]
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
    fn year(&self) -> u32 { 2022 }
    fn day(&self) -> u32 { 17 }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        Box::new(solve(input, 2022))
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        Box::new(solve(input, 1_000_000_000_000))
    }
}

fn solve(input: &str, count: u64) -> u64 {
    let jets: Vec<i32> = input.trim().chars().map(|c| if c == '>' { 1 } else { -1 }).collect();
    let mut grid = HashSet::new();
    for x in 0..7 { grid.insert((x, 0)); }
    
    let mut height = 0;
    let mut jet_idx = 0;
    let mut seen = HashMap::new();
    let mut heights = Vec::new();

    for i in 0..count {
        let rock = ROCKS[(i % 5) as usize];
        let state = (i % 5, jet_idx, get_profile(&grid, height));
        
        if let Some((prev_i, prev_h)) = seen.get(&state) {
            let cycle_len = i - prev_i;
            let cycle_h = height - prev_h;
            let remaining = count - i;
            let cycles = remaining / cycle_len;
            return height + (cycles * cycle_h) + solve_remainder(&grid, &jets[jet_idx..], i % 5, remaining % cycle_len, height);
        }
        seen.insert(state, (i, height));
        heights.push(height);

        let mut pos = (2, height + 4);
        loop {
            let jet = jets[jet_idx];
            jet_idx = (jet_idx + 1) % jets.len();
            if can_move(&grid, &rock, pos.0 + jet, pos.1) { pos.0 += jet; }
            if can_move(&grid, &rock, pos.0, pos.1 - 1) { pos.1 -= 1; }
            else {
                for (dx, dy) in rock.get_offsets() {
                    grid.insert((pos.0 + dx, pos.1 + dy), 0);
                    height = height.max(pos.1 + dy);
                }
                break;
            }
        }
    }
    height
}

fn get_profile(grid: &HashSet<(i32, i32)>, height: i32) -> Vec<i32> {
    (0..7).map(|x| (0..height).rev().find(|&y| grid.contains(&(x, y))).map_or(0, |y| height - y)).collect()
}

fn can_move(grid: &HashSet<(i32, i32)>, rock: &Rock, x: i32, y: i32) -> bool {
    rock.get_offsets().iter().all(|&(dx, dy)| {
        let nx = x + dx;
        let ny = y + dy;
        nx >= 0 && nx < 7 && ny > 0 && !grid.contains(&(nx, ny))
    })
}

fn solve_remainder(grid: &HashSet<(i32, i32)>, jets: &[i32], rock_idx: usize, count: u64, mut height: i32) -> u64 {
    let mut grid = grid.clone();
    let mut jet_idx = 0;
    for i in 0..count {
        let rock = ROCKS[(rock_idx + i as usize) % 5];
        let mut pos = (2, height + 4);
        loop {
            let jet = jets[jet_idx];
            jet_idx = (jet_idx + 1) % jets.len();
            if can_move(&grid, &rock, pos.0 + jet, pos.1) { pos.0 += jet; }
            if can_move(&grid, &rock, pos.0, pos.1 - 1) { pos.1 -= 1; }
            else {
                for (dx, dy) in rock.get_offsets() {
                    grid.insert((pos.0 + dx, pos.1 + dy), 0);
                    height = height.max(pos.1 + dy);
                }
                break;
            }
        }
    }
    (height - *grid.keys().map(|&(_, y)| y).filter(|&y| y < 0).max().unwrap_or(&0)) as u64 // This is a bit hacky, but should work for the remainder
}
