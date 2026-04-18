use anyhow::Result;
use aoc_rust_common::Solution;
use md5;

pub struct Day04;

impl Solution for Day04 {
    fn year(&self) -> u32 {
        2015
    }
    fn day(&self) -> u32 {
        4
    }

    fn part1(&self, input: &str) -> Result<String> {
        let mut counter = 0;
        let content = input.trim();
        loop {
            let data = format!("{}{}", content, counter);
            let calculated_hash = md5::compute(data);
            let hash_as_string = format!("{:x}", calculated_hash);
            if hash_as_string.starts_with("00000") {
                return Ok((counter as i64).to_string());
            }
            counter += 1;
        }
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut counter = 0;
        let content = input.trim();
        loop {
            let data = format!("{}{}", content, counter);
            let calculated_hash = md5::compute(data);
            let hash_as_string = format!("{:x}", calculated_hash);
            if hash_as_string.starts_with("000000") {
                return Ok((counter as i64).to_string());
            }
            counter += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Takes too long"]
    fn test_day04() {
        let day = Day04;
        assert_eq!(day.part1("abcdef").unwrap(), "609043");
        assert_eq!(day.part1("pqrstuv").unwrap(), "1048970");
    }
}
