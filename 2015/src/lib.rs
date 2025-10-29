pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;
pub mod day26;

/// Holds input data for an Advent of Code exercise
pub struct Exercise {
    pub content: String,
    pub example: String,
}

/// Trait for solving Advent of Code problems
pub trait Solvable {
    /// Solve part 1 of the problem
    fn first(&self, content: &str) -> i64;
    /// Solve part 2 of the problem
    fn second(&self, content: &str) -> i64;

    /// Helper to solve part 1 with example or production data
    fn solve_first(&self, is_prod: bool) -> i64;
    /// Helper to solve part 2 with example or production data
    fn solve_second(&self, is_prod: bool) -> i64;
}
