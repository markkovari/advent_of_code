use aoc_rust_common::Solution;
use std::fmt::Display;

pub struct Day01;

impl Solution for Day01 {
    fn year(&self) -> u32 {
        2015
    }
    fn day(&self) -> u32 {
        1
    }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        Box::new(input.chars().fold(0, |acc, c| match c {
            '(' => acc + 1,
            ')' => acc - 1,
            _ => acc,
        }))
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        let mut floor = 0;
        for (pos, c) in input.chars().enumerate() {
            floor += match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };
            if floor == -1 {
                return Box::new((pos + 1) as i64);
            }
        }
        Box::new((input.len() + 1) as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01() {
        let day = Day01;
        assert_eq!(day.part1("(())").to_string(), "0");
        assert_eq!(day.part1("()()").to_string(), "0");
        assert_eq!(day.part1("(((").to_string(), "3");
        assert_eq!(day.part1("(()(()(").to_string(), "3");
        assert_eq!(day.part1("))(((((").to_string(), "3");
        assert_eq!(day.part1("())").to_string(), "-1");
        assert_eq!(day.part1("))(").to_string(), "-1");
        assert_eq!(day.part1(")))").to_string(), "-3");
        assert_eq!(day.part1(")())())").to_string(), "-3");

        assert_eq!(day.part2(")").to_string(), "1");
        assert_eq!(day.part2("()())").to_string(), "5");
    }
}
