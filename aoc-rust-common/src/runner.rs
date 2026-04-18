use crate::input::get_input;
use crate::solution::Solution;
use std::time::Instant;

pub fn run_solution<S: Solution>(solution: S) {
    println!("--- Year {} Day {} ---", solution.year(), solution.day());

    let input = get_input(solution.year(), solution.day());

    let start1 = Instant::now();
    let res1 = solution.part1(&input);
    let dur1 = start1.elapsed();
    println!("Part 1: {} ({:?})", res1, dur1);

    let start2 = Instant::now();
    let res2 = solution.part2(&input);
    let dur2 = start2.elapsed();
    println!("Part 2: {} ({:?})", res2, dur2);
}
