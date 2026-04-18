use aoc_rust_common::Solution;
use rayon::prelude::*;
use std::fmt::Display;

pub struct Day05;

impl Solution for Day05 {
    fn year(&self) -> u32 {
        2015
    }
    fn day(&self) -> u32 {
        5
    }

    fn part1(&self, input: &str) -> Box<dyn Display> {
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        let filters = ["ab", "cd", "pq", "xy"];

        Box::new(
            input
                .par_lines()
                .filter(|line| line.chars().filter(|c| vowels.contains(c)).count() >= 3)
                .filter(|line| {
                    let chars: Vec<char> = line.chars().collect();
                    chars.windows(2).any(|w| w[0] == w[1])
                })
                .filter(|line| !filters.iter().any(|&f| line.contains(f)))
                .count() as i64,
        )
    }

    fn part2(&self, input: &str) -> Box<dyn Display> {
        Box::new(
            input
                .par_lines()
                .filter(|line| {
                    let chars: Vec<char> = line.chars().collect();
                    chars.windows(3).any(|w| w[0] == w[2])
                })
                .filter(|line| {
                    let chars: Vec<char> = line.chars().collect();
                    (0..chars.len() - 1).any(|i| {
                        let pair = &line[i..i + 2];
                        line[i + 2..].contains(pair)
                    })
                })
                .count() as i64,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day05() {
        let day = Day05;
        assert_eq!(day.part1("ugknbfddgicrmopn").to_string(), "1");
        assert_eq!(day.part1("aaa").to_string(), "1");
        assert_eq!(day.part1("jchzalrnumimnmhp").to_string(), "0");
        assert_eq!(day.part1("haegwjzuvuyypxyu").to_string(), "0");
        assert_eq!(day.part1("dvszwmarrgswjxmb").to_string(), "0");

        assert_eq!(day.part2("qjhvhtzxzqqjkmpb").to_string(), "1");
        assert_eq!(day.part2("xxyxx").to_string(), "1");
        assert_eq!(day.part2("uurcxstgmygtbstg").to_string(), "0");
        assert_eq!(day.part2("ieodomkazucvgmuy").to_string(), "0");
    }
}
