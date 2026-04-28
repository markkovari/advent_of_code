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

// --- Utility Functions ---

/// Calculates the layer size for a fixed image dimension.
pub const fn layer_size(width: u32, height: u32) -> u32 {
    width * height
}
