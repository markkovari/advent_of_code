use anyhow::Result;
use aoc_rust_common::Solution;

pub struct Day10;

fn look_and_say(current: &str) -> String {
    let mut result = String::with_capacity(current.len() * 2);
    let mut chars = current.chars();

    if let Some(mut current_char) = chars.next() {
        let mut current_count = 1;

        for c in chars {
            if c == current_char {
                current_count += 1;
            } else {
                result.push_str(&current_count.to_string());
                result.push(current_char);
                current_char = c;
                current_count = 1;
            }
        }
        result.push_str(&current_count.to_string());
        result.push(current_char);
    }

    result
}

impl Solution for Day10 {
    fn year(&self) -> u32 {
        2015
    }
    fn day(&self) -> u32 {
        10
    }

    fn part1(&self, input: &str) -> Result<String> {
        let mut result = input.trim().to_string();
        for _ in 0..40 {
            result = look_and_say(&result);
        }
        Ok((result.len() as i64).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut result = input.trim().to_string();
        for _ in 0..50 {
            result = look_and_say(&result);
        }
        Ok((result.len() as i64).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day10() {
        assert_eq!(look_and_say("1"), "11");
        assert_eq!(look_and_say("11"), "21");
        assert_eq!(look_and_say("21"), "1211");
        assert_eq!(look_and_say("1211"), "111221");
        assert_eq!(look_and_say("111221"), "312211");
    }
}
