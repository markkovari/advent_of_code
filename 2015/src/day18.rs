use anyhow::Result;
use aoc_rust_common::Solution;
use rayon::prelude::*;
use std::collections::HashMap;

pub struct Day18;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Cell {
    Alive,
    Dead,
}

type Grid = HashMap<(i64, i64), Cell>;

fn read_grid(content: &str) -> Grid {
    let mut grid: Grid = HashMap::new();
    for (y, line) in content.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            grid.insert(
                (x as i64, y as i64),
                if cell == '#' { Cell::Alive } else { Cell::Dead },
            );
        }
    }
    grid
}

fn get_neighbors_count(grid: &Grid, x: i64, y: i64) -> usize {
    let mut count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            if let Some(Cell::Alive) = grid.get(&(x + i, y + j)) {
                count += 1;
            }
        }
    }
    count
}

fn evolve(grid: &Grid) -> Grid {
    grid.par_iter()
        .map(|(pos, cell)| {
            let alive_neighbors = get_neighbors_count(grid, pos.0, pos.1);
            let new_cell = match cell {
                Cell::Alive => {
                    if alive_neighbors == 2 || alive_neighbors == 3 {
                        Cell::Alive
                    } else {
                        Cell::Dead
                    }
                }
                Cell::Dead => {
                    if alive_neighbors == 3 {
                        Cell::Alive
                    } else {
                        Cell::Dead
                    }
                }
            };
            (*pos, new_cell)
        })
        .collect()
}

fn setup_corners(grid: &mut Grid) {
    let max_x = grid.keys().map(|e| e.0).max().unwrap();
    let max_y = grid.keys().map(|e| e.1).max().unwrap();
    grid.insert((0, 0), Cell::Alive);
    grid.insert((0, max_y), Cell::Alive);
    grid.insert((max_x, 0), Cell::Alive);
    grid.insert((max_x, max_y), Cell::Alive);
}

impl Solution for Day18 {
    fn year(&self) -> u32 {
        2015
    }
    fn day(&self) -> u32 {
        18
    }

    fn part1(&self, input: &str) -> Result<String> {
        let mut grid = read_grid(input);
        let steps = if grid.len() < 100 { 4 } else { 100 };
        for _ in 0..steps {
            grid = evolve(&grid);
        }
        Ok((grid.values().filter(|e| **e == Cell::Alive).count() as i64).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut grid = read_grid(input);
        let steps = if grid.len() < 100 { 5 } else { 100 };
        for _ in 0..steps {
            setup_corners(&mut grid);
            grid = evolve(&grid);
        }
        setup_corners(&mut grid);
        Ok((grid.values().filter(|e| **e == Cell::Alive).count() as i64).to_string())
    }
}
