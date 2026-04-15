use aoc_rust_common::Solution;
use std::fmt::Display;
use rayon::prelude::*;

pub struct Day02;

fn calculate_wrapper_needed_materials(line: &str) -> (i64, i64) {
    let dimensions = line
        .split('x')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let l = dimensions[0];
    let w = dimensions[1];
    let h = dimensions[2];
    
    let areas = [l * w, w * h, h * l];
    let smallest = areas.iter().min().unwrap();
    
    let perimeters = [2 * (l + w), 2 * (w + h), 2 * (h + l)];
    let smallest_perimeter = perimeters.iter().min().unwrap();

    (
        2 * l * w + 2 * w * h + 2 * h * l + smallest,
        smallest_perimeter + l * w * h,
    )
}

impl Solution for Day02 {
    fn year(&self) -> u32 { 2015 }
    fn day(&self) -> u32 { 2 }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        Box::new(input
            .par_lines()
            .map(calculate_wrapper_needed_materials)
            .map(|(wrapping, _)| wrapping)
            .sum::<i64>())
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        Box::new(input
            .par_lines()
            .map(calculate_wrapper_needed_materials)
            .map(|(_, ribbon)| ribbon)
            .sum::<i64>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day02() {
        let day = Day02;
        assert_eq!(day.part1("2x3x4").to_string(), "58");
        assert_eq!(day.part1("1x1x10").to_string(), "43");
        
        assert_eq!(day.part2("2x3x4").to_string(), "34");
        assert_eq!(day.part2("1x1x10").to_string(), "14");
    }
}
