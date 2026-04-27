use anyhow::Result;

/// The core trait for an Advent of Code solution.
///
/// Every puzzle implementation should implement this trait to integrate with the
/// common runner and benchmarking infrastructure.
pub trait Solution {
    /// Returns the year of the puzzle.
    fn year(&self) -> u32;
    /// Returns the day of the puzzle.
    fn day(&self) -> u32;

    /// Solves the first part of the puzzle.
    fn part1(&self, input: &str) -> Result<String>;
    /// Solves the second part of the puzzle.
    fn part2(&self, input: &str) -> Result<String>;
}
