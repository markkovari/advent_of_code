use aoc_rust_common::run_solution;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Year of the puzzle
    year: u32,
    /// Day of the puzzle
    day: u32,
}

fn main() {
    let args = Args::parse();

    match args.year {
        2015 => run_2015(args.day),
        2018 => run_2018(args.day),
        2019 => run_2019(args.day),
        2020 => run_2020(args.day),
        2021 => run_2021(args.day),
        2022 => run_2022(args.day),
        2025 => run_2025(args.day),
        _ => eprintln!("Year {} not supported yet.", args.year),
    }
}

fn run_2015(day: u32) {
    use aoc_rust_2015::*;
    match day {
        1 => run_solution(day01::Day01),
        2 => run_solution(day02::Day02),
        3 => run_solution(day03::Day03),
        4 => run_solution(day04::Day04),
        5 => run_solution(day05::Day05),
        6 => run_solution(day06::Day06),
        7 => run_solution(day07::Day07),
        8 => run_solution(day08::Day08),
        9 => run_solution(day09::Day09),
        10 => run_solution(day10::Day10),
        11 => run_solution(day11::Day11),
        12 => run_solution(day12::Day12),
        13 => run_solution(day13::Day13),
        14 => run_solution(day14::Day14),
        15 => run_solution(day15::Day15),
        16 => run_solution(day16::Day16),
        17 => run_solution(day17::Day17),
        18 => run_solution(day18::Day18),
        19 => run_solution(day19::Day19),
        20 => run_solution(day20::Day20),
        21 => run_solution(day21::Day21),
        22 => run_solution(day22::Day22),
        23 => run_solution(day23::Day23),
        24 => run_solution(day24::Day24),
        25 => run_solution(day25::Day25),
        26 => run_solution(day26::Day26),
        _ => eprintln!("2015 Day {} not found.", day),
    }
}

fn run_2018(day: u32) {
    use aoc_rust_2018::*;
    match day {
        1 => run_solution(day01::Day01),
        2 => run_solution(day02::Day02),
        3 => run_solution(day03::Day03),
        4 => run_solution(day04::Day04),
        5 => run_solution(day05::Day05),
        6 => run_solution(day06::Day06),
        7 => run_solution(day07::Day07),
        8 => run_solution(day08::Day08),
        9 => run_solution(day09::Day09),
        10 => run_solution(day10::Day10),
        11 => run_solution(day11::Day11),
        12 => run_solution(day12::Day12),
        13 => run_solution(day13::Day13),
        14 => run_solution(day14::Day14),
        15 => run_solution(day15::Day15),
        16 => run_solution(day16::Day16),
        17 => run_solution(day17::Day17),
        18 => run_solution(day18::Day18),
        19 => run_solution(day19::Day19),
        20 => run_solution(day20::Day20),
        21 => run_solution(day21::Day21),
        22 => run_solution(day22::Day22),
        23 => run_solution(day23::Day23),
        24 => run_solution(day24::Day24),
        25 => run_solution(day25::Day25),
        _ => eprintln!("2018 Day {} not found.", day),
    }
}

fn run_2019(day: u32) {
    use aoc_rust_2019::days::*;
    match day {
        13 => run_solution(day13::Day13),
        14 => run_solution(day14::Day14),
        _ => eprintln!("2019 Day {} not found in aoc_rust_2019.", day),
    }
}

fn run_2020(day: u32) {
    use aoc_rust_2020::*;
    match day {
        1 => run_solution(day01::Day01),
        _ => eprintln!("2020 Day {} not found.", day),
    }
}

fn run_2021(day: u32) {
    match day {
        1 => run_solution(aoc_rust_2021::day01::Day01),
        2 => run_solution(aoc_rust_2021::day02::Day02),
        3 => run_solution(aoc_rust_2021::day03::Day03),
        4 => run_solution(aoc_rust_2021::day04::Day04),
        5 => run_solution(aoc_rust_2021::day05::Day05),
        6 => run_solution(aoc_rust_2021::day06::Day06),
        7 => run_solution(aoc_rust_2021::day07::Day07),
        _ => eprintln!("2021 Day {} not found.", day),
    }
}

fn run_2022(day: u32) {
    use aoc_rust_2022::*;
    match day {
        1 => run_solution(day01::Day01),
        2 => run_solution(day02::Day02),
        3 => run_solution(day03::Day03),
        4 => run_solution(day04::Day04),
        5 => run_solution(day05::Day05),
        6 => run_solution(day06::Day06),
        7 => run_solution(day07::Day07),
        8 => run_solution(day08::Day08),
        9 => run_solution(day09::Day09),
        10 => run_solution(day10::Day10),
        11 => run_solution(day11::Day11),
        12 => run_solution(day12::Day12),
        13 => run_solution(day13::Day13),
        14 => run_solution(day14::Day14),
        15 => run_solution(day15::Day15),
        16 => run_solution(day16::Day16),
        17 => run_solution(day17::Day17),
        18 => run_solution(day18::Day18),
        19 => run_solution(day19::Day19),
        25 => run_solution(day25::Day25),
        _ => eprintln!("2022 Day {} not found.", day),
    }
}

fn run_2025(day: u32) {
    use aoc_rust_2025::*;
    match day {
        1 => run_solution(day01::Day01),
        2 => run_solution(day02::Day02),
        _ => eprintln!("2025 Day {} not found.", day),
    }
}
