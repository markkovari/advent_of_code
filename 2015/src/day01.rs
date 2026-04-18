use anyhow::Result;
use aoc_rust_common::Solution;

pub struct Day01;

impl Solution for Day01 {
    fn year(&self) -> u32 {
        2015
    }
    fn day(&self) -> u32 {
        1
    }

    fn part1(&self, input: &str) -> Result<String> {
        Ok(input
            .chars()
            .fold(0, |acc, c| match c {
                '(' => acc + 1,
                ')' => acc - 1,
                _ => acc,
            })
            .to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut floor = 0;
        for (pos, c) in input.chars().enumerate() {
            floor += match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };
            if floor == -1 {
                return Ok(((pos + 1) as i64).to_string());
            }
        }
        Ok(((input.len() + 1) as i64).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01() {
        let day = Day01;
        assert_eq!(day.part1("(())").unwrap(), "0");
        assert_eq!(day.part1("()()").unwrap(), "0");
        assert_eq!(day.part1("(((").unwrap(), "3");
        assert_eq!(day.part1("(()(()(").unwrap(), "3");
        assert_eq!(day.part1("))(((((").unwrap(), "3");
        assert_eq!(day.part1("())").unwrap(), "-1");
        assert_eq!(day.part1("))(").unwrap(), "-1");
        assert_eq!(day.part1(")))").unwrap(), "-3");
        assert_eq!(day.part1(")())())").unwrap(), "-3");

        assert_eq!(day.part2(")").unwrap(), "1");
        assert_eq!(day.part2("()())").unwrap(), "5");
    }
}
