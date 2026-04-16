use aoc_rust_common::Solution;
use std::fmt::Display;

pub struct Day08;

fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()).collect()
}

fn visible(grid: &[Vec<u8>], r: usize, c: usize) -> bool {
    let height = grid[r][c];
    let rows = grid.len();
    let cols = grid[0].len();

    let up = (0..r).all(|i| grid[i][c] < height);
    let down = (r + 1..rows).all(|i| grid[i][c] < height);
    let left = (0..c).all(|i| grid[r][i] < height);
    let right = (c + 1..cols).all(|i| grid[r][i] < height);

    up || down || left || right
}

fn score(grid: &[Vec<u8>], r: usize, c: usize) -> u64 {
    let height = grid[r][c];
    let rows = grid.len();
    let cols = grid[0].len();

    let mut up = 0;
    for i in (0..r).rev() {
        up += 1;
        if grid[i][c] >= height { break; }
    }
    let mut down = 0;
    for i in r + 1..rows {
        down += 1;
        if grid[i][c] >= height { break; }
    }
    let mut left = 0;
    for i in (0..c).rev() {
        left += 1;
        if grid[r][i] >= height { break; }
    }
    let mut right = 0;
    for i in c + 1..cols {
        right += 1;
        if grid[r][i] >= height { break; }
    }
    up * down * left * right
}

impl Solution for Day08 {
    fn year(&self) -> u32 { 2022 }
    fn day(&self) -> u32 { 8 }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let grid = parse(input);
        let rows = grid.len();
        let cols = grid[0].len();
        let count = (0..rows).flat_map(|r| (0..cols).map(move |c| (r, c)))
            .filter(|&(r, c)| visible(&grid, r, c))
            .count();
        Box::new(count)
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let grid = parse(input);
        let rows = grid.len();
        let cols = grid[0].len();
        let max_score = (0..rows).flat_map(|r| (0..cols).map(move |c| (r, c)))
            .map(|(r, c)| score(&grid, r, c))
            .max().unwrap_or(0);
        Box::new(max_score)
    }
}
