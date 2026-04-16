mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
use aoc_rust_common::run_solution;
use day01::Day01;
use day02::Day02;
use day03::Day03;
use day04::Day04;
use day05::Day05;

fn main() {
    run_solution(Day01);
    run_solution(Day02);
    run_solution(Day03);
    run_solution(Day04);
    run_solution(Day05);
}
