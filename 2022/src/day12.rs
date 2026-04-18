use anyhow::Result;
use aoc_rust_common::Solution;
use std::collections::{HashSet, VecDeque};

pub struct Day12;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pos {
    r: usize,
    c: usize,
}

fn parse(input: &str) -> (Vec<Vec<u8>>, Pos, Pos) {
    let mut start = Pos { r: 0, c: 0 };
    let mut end = Pos { r: 0, c: 0 };
    let grid = input
        .lines()
        .enumerate()
        .map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(|(c, ch)| match ch {
                    'S' => {
                        start = Pos { r, c };
                        0
                    }
                    'E' => {
                        end = Pos { r, c };
                        25
                    }
                    _ => ch as u8 - b'a',
                })
                .collect()
        })
        .collect();
    (grid, start, end)
}

fn bfs(grid: &Vec<Vec<u8>>, start: Pos, end: Option<Pos>, find_min: bool) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    let mut visited = HashSet::new();
    visited.insert(start);

    while let Some((pos, steps)) = queue.pop_front() {
        if let Some(e) = end {
            if pos == e {
                return steps;
            }
        } else if !find_min && grid[pos.r][pos.c] == 0 {
            return steps;
        }

        let current_h = grid[pos.r][pos.c];
        for (dr, dc) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nr = pos.r as isize + dr;
            let nc = pos.c as isize + dc;
            if nr >= 0 && nr < grid.len() as isize && nc >= 0 && nc < grid[0].len() as isize {
                let npos = Pos {
                    r: nr as usize,
                    c: nc as usize,
                };
                let next_h = grid[npos.r][npos.c];
                if !visited.contains(&npos)
                    && (find_min && next_h <= current_h + 1 || !find_min && current_h <= next_h + 1)
                {
                    visited.insert(npos);
                    queue.push_back((npos, steps + 1));
                }
            }
        }
    }
    usize::MAX
}

impl Solution for Day12 {
    fn year(&self) -> u32 {
        2022
    }
    fn day(&self) -> u32 {
        12
    }

    fn part1(&self, input: &str) -> Result<String> {
        let (grid, start, end) = parse(input);
        Ok((bfs(&grid, start, Some(end), true)).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let (grid, _, end) = parse(input);
        Ok((bfs(&grid, end, None, false)).to_string())
    }
}
